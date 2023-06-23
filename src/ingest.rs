use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
    time::Duration,
};

use anyhow::Context;
use chrono::{DateTime, Utc};
use futures::{pin_mut, Stream, StreamExt};
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;

use crate::config;

/// The per-station buffers of weather data sorted by creation time
type Buffers = HashMap<u8, VecDeque<WeatherMessage>>;

/// The error type produced by the mock input stream
type Error = anyhow::Error;

/// A group of messages
pub type MessageGroup = HashMap<u8, Option<WeatherMessage>>;

/// A single message from a weather station
#[derive(Debug)]
pub struct WeatherMessage {
    /// The ID of the station this message came from
    pub station_id: u8,
    /// The time which this message was created
    pub created_at: DateTime<Utc>,
    /// The time at which our application received this message
    pub received_at: DateTime<Utc>,
    /// The temperature measured at the time the message was created
    pub temp: f32,
    /// The barometric pressure at the time the message was created
    pub pressure: f32,
    /// The wind speed in m/s at the time the message was created
    pub wind_speed: f32,
    /// The wind direction in degrees relative to North at the time the message was created
    pub wind_direction: f32,
}

/// Demux and buffer the incoming messages
pub async fn demux(
    stream: impl Stream<Item = Result<WeatherMessage, Error>>,
    synced_buffers: Arc<Mutex<Buffers>>,
) -> Result<(), Error> {
    pin_mut!(stream); // StreamExt::next requires that the stream be Unpin
    while let Some(Ok(msg)) = stream.next().await {
        let mut buffers = synced_buffers.lock().await;
        insert_message(msg, &mut buffers)
            .await
            .context("failed to insert message")?;
    }
    Ok(())
}

/// Insert a new message into the buffers maintaining sorted order
async fn insert_message(msg: WeatherMessage, buffers: &mut Buffers) -> Result<(), Error> {
    // Get the buffer for this particular weather station
    let buffer = buffers
        .get_mut(&msg.station_id)
        .context("tried to insert message with unknown station id")?;
    // Ensure that we aren't writing to a full buffer. If the buffer is full we'll just naively wait.
    // You absolutely shouldn't do this in real life, use a real backpressure mechanism instead, like
    // evicting and "nack"ing data you can't use yet.
    while buffer.len() > config::BUFFER_SIZE {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    // Find the place to insert the new message that will maintain sorted order, then insert it
    let insertion_point = buffer.binary_search_by_key(&msg.created_at, |m| m.created_at);
    match insertion_point {
        Ok(_) => {
            // If we find this exact `created_at` for this weather station it means we received
            // a duplicate, in which case we can just keep the message that's already in the buffer
            // and discard the one we just received.
            Ok(())
        }
        Err(i) => {
            // Insert the message into the correct position to maintain sorted order
            buffer.insert(i, msg);
            Ok(())
        }
    }
}

/// Group the latest messages once per `GROUPING_TIMEOUT`
pub async fn group_latest(
    synced_buffers: Arc<Mutex<Buffers>>,
    chan: Sender<MessageGroup>,
) -> Result<(), Error> {
    let mut interval = tokio::time::interval(config::GROUPING_TIMEOUT);
    loop {
        interval.tick().await;
        let mut buffers = synced_buffers.lock().await;
        // TODO: Do more sophisticated grouping later
        let grouped_msgs = buffers
            .iter_mut()
            .map(|(station_number, buffer)| (*station_number, buffer.pop_front()))
            .collect::<HashMap<_, _>>();
        chan.send(grouped_msgs)
            .await
            .context("couldn't send message group")?;
    }
    Ok(())
}

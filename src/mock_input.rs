use chrono::{DateTime, Utc};
use futures::Stream;

use crate::{config, ingest::WeatherMessage};

type Error = anyhow::Error;

/// Returns a stream producing mock incoming weather data
pub fn incoming_weather_data() -> impl Stream<Item = Result<WeatherMessage, Error>> {
    let creation_start_time = chrono::offset::Utc::now()
        - chrono::Duration::from_std(config::MESSAGE_PRODUCTION_LATENCY).unwrap();
    async_stream::stream! {
        // Produce a message from each station
        // Sleep until the next interval
        let mut interval = tokio::time::interval(config::MESSAGE_PRODUCTION_PERIOD);
        loop {
            for station_number in 0..config::N_WEATHER_STATIONS {
                let msg = new_message(station_number);
                // TODO: Add a probability to drop a message here to simulate network conditions
                yield Ok(msg);
            }
            interval.tick().await;
        }
    }
}

/// Returns the mock time at which the message was created
fn created_at() -> DateTime<Utc> {
    chrono::offset::Utc::now()
        - chrono::Duration::from_std(config::MESSAGE_PRODUCTION_LATENCY).unwrap()
}

/// Returns the time at which the message was received
fn received_at() -> DateTime<Utc> {
    chrono::offset::Utc::now()
}

/// Produce a new message filled with dummy data
fn new_message(station_number: u8) -> WeatherMessage {
    WeatherMessage {
        station_id: station_number,
        created_at: created_at(),
        received_at: received_at(),
        temp: rand::random::<f32>(),
        pressure: rand::random::<f32>(),
        wind_speed: rand::random::<f32>(),
        wind_direction: rand::random::<f32>(),
    }
}

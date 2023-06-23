use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use futures::{select, FutureExt};
use ingest::MessageGroup;
use publish::WeatherPrediction;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

mod config;
mod ingest;
mod mock_input;
mod publish;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let incoming_messages = mock_input::incoming_weather_data();
    let inner_buffers = (0..config::N_WEATHER_STATIONS)
        .map(|i| (i, VecDeque::new()))
        .collect::<HashMap<_, _>>();
    let buffers = Arc::new(Mutex::new(inner_buffers));
    let mut demuxer = ingest::demux(incoming_messages, buffers.clone())
        .boxed()
        .fuse();
    let (group_sender, group_receiver) = tokio::sync::mpsc::channel(config::PROCESSING_BUFFER_SIZE);
    let mut grouper = ingest::group_latest(buffers.clone(), group_sender)
        .boxed()
        .fuse();
    let (pred_sender, pred_receiver) = tokio::sync::mpsc::channel(config::PROCESSING_BUFFER_SIZE);
    let mut processor = really_sophisticated_weather_analysis(group_receiver, pred_sender)
        .boxed()
        .fuse();
    let mut publisher = publish::publish(pred_receiver).boxed().fuse();

    select! {
        res = demuxer => res,
        res = grouper => res,
        res = processor => res,
        res = publisher => res,
    }
}

/// Perform detailed analysis of atmospheric data to predict the weather
async fn really_sophisticated_weather_analysis(
    mut receiver: Receiver<MessageGroup>,
    sender: Sender<WeatherPrediction>,
) -> Result<(), anyhow::Error> {
    while let Some(group) = receiver.recv().await {
        let temps = group
            .values()
            .filter(|msg| msg.is_some())
            .map(|msg| msg.as_ref().unwrap().temp)
            .collect::<Vec<_>>();
        if temps.is_empty() {
            let prediction = WeatherPrediction { chance_rain: 0.0 };
            sender.send(prediction).await?;
        }
        // Can't use `temps.iter().max()` because `f32`/`f64` doesn't implement `Ord` due to NaN
        let max_temp: f32 = temps.clone().into_iter().reduce(|x, y| x.max(y)).unwrap();
        let n_temps = f32::from(temps.iter().len() as u16); // Surely we have fewer than 65536 items...right?
        let avg_temp: f32 = temps.into_iter().sum::<f32>() / n_temps;
        let prediction = WeatherPrediction {
            chance_rain: avg_temp / max_temp,
        };
        sender.send(prediction).await?;
    }
    Ok(())
}

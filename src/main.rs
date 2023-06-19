use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

mod config;
mod ingest;
mod mock_input;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let incoming_messages = mock_input::incoming_weather_data();
    let mut buffers = Arc::new(Mutex::new(HashMap::new()));
    let demuxer = ingest::demux(incoming_messages, buffers.clone());
    let (sender, receiver) = tokio::sync::mpsc::channel(config::PROCESSING_BUFFER_SIZE);
    let grouper = ingest::group_latest(buffers.clone(), sender);
}

mod config;
mod ingest;
mod mock_input;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let incoming_messages = mock_input::incoming_weather_data();
}

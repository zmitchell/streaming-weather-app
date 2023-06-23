use tokio::sync::mpsc::Receiver;

type Error = anyhow::Error;

#[derive(Debug)]
pub struct WeatherPrediction {
    pub chance_rain: f32,
}

/// Publishes weather predictions to mock PubSub
pub async fn publish(mut receiver: Receiver<WeatherPrediction>) -> Result<(), Error> {
    while let Some(pred) = receiver.recv().await {
        println!("Chance of rain: {:.1}%", pred.chance_rain * 100.0);
    }
    Ok(())
}

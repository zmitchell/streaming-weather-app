use chrono::{DateTime, Utc};

/// A single message from a weather station
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

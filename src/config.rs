use std::time::Duration;

// The time interval at which messages are produced by a given weather station
pub const MESSAGE_PRODUCTION_PERIOD: Duration = Duration::from_millis(1000);

// The difference in time between message creation and message arrival
pub const MESSAGE_PRODUCTION_LATENCY: Duration = Duration::from_millis(5 * 1000);

// The number of weather stations to use in the simulation
pub const N_WEATHER_STATIONS: u8 = 10;

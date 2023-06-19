use std::time::Duration;

// The time interval at which messages are produced by a given weather station
pub const MESSAGE_PRODUCTION_PERIOD: Duration = Duration::from_millis(1000);

// The difference in time between message creation and message arrival
pub const MESSAGE_PRODUCTION_LATENCY: Duration = Duration::from_millis(5 * 1000);

// The number of weather stations to use in the simulation
pub const N_WEATHER_STATIONS: u8 = 10;

// The number of messages that should be stored in each buffer
pub const BUFFER_SIZE: usize = 100;

// How long to wait before producing the largest group available
pub const GROUPING_TIMEOUT: Duration = Duration::from_millis(1100);

// Messages falling into a window of this size will be considered a valid group
pub const GROUPING_WINDOW: Duration = Duration::from_millis(1100);

// The size of the channel connecting the inputs and outputs of the processor task
pub const PROCESSING_BUFFER_SIZE: usize = 100;

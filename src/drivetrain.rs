use serde::{Serialize, Deserialize};

// struct holding the current* value of the encoders
// * up to 1 second in lag as this occurs on a 1hz update timer
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
pub struct MotorCounts {
    // these two have 32 bit resolution due to their timer
    pub north_west: MotorDelta,
    pub north_east: MotorDelta,
    // different timer, which only has 16 bit resolution
    pub south_east: MotorDelta,
    pub south_west: MotorDelta,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
pub struct MotorDelta {
    pub count: u32,
    pub delta: i64,
}


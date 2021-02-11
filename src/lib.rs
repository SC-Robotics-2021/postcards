#![no_std]
use postcard::{flavors, serialize_with_flavor};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
// Request kind
pub enum RequestKind {
    // Sets all Drive motors to target speed. Target must be within [-1.0, 1.0]
    SetSpeed { target: f32 },
    // Sets the left side of the drive motors to target speed.
    // Target must be within [-1.0, 1.0].
    SetLeftSpeed { target: f32 },
    // Sets the right side of the drive motors to the target speed.
    // Target must be within [-1.0, 1.0]
    SetRightSpeed { target: f32 },
    // Sets the two different drivetrain sides in the same message.
    SetSplitSpeed { left: f32, right: f32 },
    // Stop all drive actuators
    HaltMotors,
    // Halt all arm actuators
    HaltArm,
    // Halt both the arm and the drive actuators.
    Halt,
    // Set the arm to a position.
    SetArm,
    // Get the encoder counts.
    GetMotorEncoderCounts,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Status {
    // MCU is OK with this query and completed it successfully
    OK,
    // Runtime handling error.
    ERROR,
    // Object failed to decode, no state can be returned.
    DecodeError,
    // Not implemented (yet?).
    Unimplemented,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
// MCU action request, user-provided state is returned in the Response object (requires decode)
pub struct Request {
    pub(crate) kind: RequestKind,
    pub(crate) state: i32,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[non_exhaustive]
/// The kind of response, this is marked non-exhaustive to allow expansion to the protocol
pub enum ResponseKind{
    /// A motor response
    MotorCountResponse(MotorCounts)

}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
// MCU action response, can include up to 256 bytes of data and will include request-provided state
// if the object successfully decoded; -1 otherwise.
pub struct Response {
    pub(crate) status: Status,
    pub(crate) state: i32,
    pub(crate) data: Option<ResponseKind>,
}

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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

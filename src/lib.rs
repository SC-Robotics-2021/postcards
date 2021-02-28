#![no_std]

pub mod arm;
pub mod drivetrain;

use serde::{Deserialize, Serialize};

pub use arm::KinematicArmPose;
pub use drivetrain::{MotorCounts, MotorDelta};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[non_exhaustive]
// Request kind
pub enum RequestKind {
    /// Sets all Drive motors to target speed. Target must be within [-1.0, 1.0]
    SetSpeed { target: f32 },
    /// Sets the left side of the drive motors to target speed.
    /// Target must be within [-1.0, 1.0].
    SetLeftSpeed { target: f32 },
    /// Sets the right side of the drive motors to the target speed.
    /// Target must be within [-1.0, 1.0]
    SetRightSpeed { target: f32 },
    /// Sets the two different drivetrain sides in the same message.
    SetSplitSpeed { left: f32, right: f32 },
    /// Stop all drive actuators
    HaltMotors,
    /// Halt all arm actuators
    HaltArm,
    /// Halt both the arm and the drive actuators.
    Halt,
    /// Set the arm to a position.
    SetArm,
    /// Get the encoder counts.
    GetMotorEncoderCounts,
    /// gets the state of the kinematic model arm
    GetKinematicArmPose,
    /// Sets the arm from a provided pose
    SetArmPose(KinematicArmPose)
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
    pub kind: RequestKind,
    pub state: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[non_exhaustive]
/// The kind of response, this is marked non-exhaustive to allow expansion to the protocol
pub enum ResponseKind{
    /// A motor response
    MotorCountResponse(MotorCounts),
    /// Kinematic model arm pose
    KinematicArmPose(KinematicArmPose),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
// MCU action response, can include up to 256 bytes of data and will include request-provided state
// if the object successfully decoded; -1 otherwise.
pub struct Response {
    pub status: Status,
    pub state: i32,
    pub data: Option<ResponseKind>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

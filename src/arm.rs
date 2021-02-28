use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Default)]
pub struct KinematicArmPose {
    pub lower_axis: Option<f32>,
    pub upper_axis: Option<f32>,
    pub rotation_axis: Option<f32>,
    pub grip: Option<GripperPose>
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct GripperPose {
    pub rotation_axis: Option<f32>,
    pub gripper_axis: Option<f32>,
    pub pitch_axis: Option<f32>,
}
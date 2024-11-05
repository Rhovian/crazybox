pub const DRONE_MASS: f32 = 0.027; // 27g
pub const GRAVITY: f32 = 9.81;
pub const HOVER_THRUST: f32 = DRONE_MASS * GRAVITY;
pub const MAX_THRUST_PER_MOTOR: f32 = HOVER_THRUST / 2.0; // Each motor needs to provide 1/4 of hover thrust
pub const BASE_THROTTLE: f32 = 0.5; // 50% throttle should hover

pub const HEIGHT_P_GAIN: f32 = 0.5;
pub const HEIGHT_D_GAIN: f32 = 0.2;

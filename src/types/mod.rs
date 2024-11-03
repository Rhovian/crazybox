use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct DroneState {
    pub roll: f32,   // degrees
    pub pitch: f32,  // degrees
    pub yaw: f32,    // degrees
    pub thrust: u16, // 0-65535
    pub armed: bool,
    pub battery_voltage: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RpytCommand {
    pub roll: f32,   // degrees
    pub pitch: f32,  // degrees
    pub yaw: f32,    // degrees/sec
    pub thrust: u16, // 0-65535
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DroneCommand {
    Rpyt(RpytCommand),
    Arm,    // Sends zero thrust to unlock
    Disarm, // Stops motors
}

#[async_trait]
pub trait DroneInterface: Send + Sync {
    async fn init(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_state(&self) -> Result<DroneState, Box<dyn std::error::Error>>;
    async fn send_command(&mut self, cmd: DroneCommand) -> Result<(), Box<dyn std::error::Error>>;
}

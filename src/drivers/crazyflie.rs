use crazyflie_lib::Crazyflie;
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::Result;
use types::{DroneInterface, DroneState, DroneCommand, RpytCommand};

pub struct CrazyflieDriver {
    cf: Arc<Crazyflie>,
    state: Arc<Mutex<DroneState>>,
}

impl CrazyflieDriver {
    pub async fn new(uri: &str) -> Result<Self> {
        let link_ctx = crazyflie_lib::LinkContext::new();
        let cf = Arc::new(Crazyflie::connect_from_uri(&link_ctx, uri).await?);
        
        // Set up logging for state updates
        let mut block = cf.log.create_block().await?;
        block.add_variable("stabilizer.roll").await?;
        block.add_variable("stabilizer.pitch").await?;
        block.add_variable("stabilizer.yaw").await?;
        block.add_variable("pm.vbat").await?;
        
        let period = crazyflie_lib::LogPeriod::from_millis(10)?; // 100Hz
        let mut stream = block.start(period).await?;
        
        // Start state update task
        let state = Arc::new(Mutex::new(DroneState::default()));
        let state_clone = state.clone();
        tokio::spawn(async move {
            while let Ok(data) = stream.next().await {
                let mut state = state_clone.lock().await;
                if let Some(value) = data.get("stabilizer.roll") {
                    state.roll = value.as_f32().unwrap_or(0.0);
                }
                if let Some(value) = data.get("stabilizer.pitch") {
                    state.pitch = value.as_f32().unwrap_or(0.0);
                }
                if let Some(value) = data.get("stabilizer.yaw") {
                    state.yaw = value.as_f32().unwrap_or(0.0);
                }
                if let Some(value) = data.get("pm.vbat") {
                    state.battery_voltage = value.as_f32().unwrap_or(0.0);
                }
            }
        });

        Ok(Self { cf, state })
    }
}

impl DroneInterface for CrazyflieDriver {
    async fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Safety: Send initial zero thrust to unlock
        self.cf.commander.setpoint_rpyt(0.0, 0.0, 0.0, 0).await?;
        Ok(())
    }
    
    async fn get_state(&self) -> Result<DroneState, Box<dyn std::error::Error>> {
        Ok(self.state.lock().await.clone())
    }
    
    async fn send_command(&mut self, cmd: DroneCommand) -> Result<(), Box<dyn std::error::Error>> {
        match cmd {
            DroneCommand::Rpyt(RpytCommand { roll, pitch, yaw, thrust }) => {
                self.cf.commander.setpoint_rpyt(roll, pitch, yaw, thrust).await?;
                let mut state = self.state.lock().await;
                state.thrust = thrust;
                state.armed = thrust > 0;
            },
            DroneCommand::Arm => {
                self.cf.commander.setpoint_rpyt(0.0, 0.0, 0.0, 0).await?;
                let mut state = self.state.lock().await;
                state.armed = true;
            },
            DroneCommand::Disarm => {
                self.cf.commander.setpoint_rpyt(0.0, 0.0, 0.0, 0).await?;
                let mut state = self.state.lock().await;
                state.armed = false;
            }
        }
        Ok(())
    }
}
use crate::types::{DroneInterface, DroneState, DroneCommand, RpytCommand};
use crate::sim::SimulationPlugin;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct SimulationDriver {
    state: Arc<Mutex<DroneState>>,
    command_tx: mpsc::Sender<DroneCommand>,
}

impl SimulationDriver {
    pub async fn new() -> anyhow::Result<Self> {
        let (command_tx, command_rx) = mpsc::channel(32);
        let state = Arc::new(Mutex::new(DroneState::default()));
        let state_clone = state.clone();

        // Spawn Bevy app in separate thread
        std::thread::spawn(move || {
            App::new()
                .add_plugins(DefaultPlugins)
                .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
                .add_plugins(RapierDebugRenderPlugin::default()) // Optional: for visualization
                .add_plugins(SimulationPlugin::new(command_rx, state_clone))
                .run();
        });

        Ok(Self {
            state,
            command_tx,
        })
    }
}

#[async_trait]
impl DroneInterface for SimulationDriver {
    async fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.command_tx
            .send(DroneCommand::Rpyt(RpytCommand {
                roll: 0.0,
                pitch: 0.0,
                yaw: 0.0,
                thrust: 0,
            }))
            .await?;
        Ok(())
    }

    async fn get_state(&self) -> Result<DroneState, Box<dyn std::error::Error>> {
        Ok(self.state.lock().await.clone())
    }

    async fn send_command(&mut self, cmd: DroneCommand) -> Result<(), Box<dyn std::error::Error>> {
        self.command_tx.send(cmd).await?;
        Ok(())
    }
}

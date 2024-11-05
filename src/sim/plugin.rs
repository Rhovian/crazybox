use crate::types::{DroneCommand, DroneState};
use bevy::prelude::*;
use bevy_rapier3d::{
    plugin::{RapierConfiguration, TimestepMode},
    prelude::Velocity,
};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use super::{
    drone::{
        apply_motor_forces, calculate_motor_throttles, height_control, setup_drone, Drone,
        HeightController,
    },
    environment::setup_environment,
    state::{update_state_sync, SimStateSync},
};

#[derive(Resource)]
pub struct SimCommandQueue(pub Arc<Mutex<mpsc::Receiver<DroneCommand>>>);
pub struct SimulationPlugin {
    command_rx: Arc<Mutex<mpsc::Receiver<DroneCommand>>>,
    state: Arc<Mutex<DroneState>>,
}

impl SimulationPlugin {
    pub fn new(command_rx: mpsc::Receiver<DroneCommand>, state: Arc<Mutex<DroneState>>) -> Self {
        Self {
            command_rx: Arc::new(Mutex::new(command_rx)),
            state,
        }
    }
}

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimCommandQueue(self.command_rx.clone()))
            .insert_resource(SimStateSync(self.state.clone()))
            .insert_resource(RapierConfiguration {
                timestep_mode: TimestepMode::Fixed {
                    dt: 1.0 / 60.0,
                    substeps: 1,
                },
                gravity: Vec3::new(0.0, -9.81, 0.0),
                physics_pipeline_active: true,
                query_pipeline_active: true,
                scaled_shape_subdivision: 10,
                force_update_from_transform_changes: true,
            })
            .add_systems(Startup, (setup_drone, setup_environment))
            .add_systems(
                Update,
                (
                    height_control,
                    process_commands,
                    apply_motor_forces,
                    update_state_sync,
                )
                    .chain(),
            );
    }
}

fn process_commands(
    command_queue: ResMut<SimCommandQueue>,
    mut drone_query: Query<&mut Drone>,
    mut height_query: Query<(&mut HeightController, &Transform, &Velocity)>,
) {
    if let Ok(mut drone) = drone_query.get_single_mut() {
        let height_correction =
            if let Ok((mut controller, transform, velocity)) = height_query.get_single_mut() {
                let error = controller.target - transform.translation.y;
                // Simple P controller with velocity damping
                let correction = error * 0.5 + (-velocity.linvel.y * 0.2);
                correction.clamp(-0.3, 0.3)
            } else {
                0.0
            };

        if let Ok(mut receiver) = command_queue.0.try_lock() {
            while let Ok(command) = receiver.try_recv() {
                match command {
                    DroneCommand::Rpyt(cmd) => {
                        let throttles = calculate_motor_throttles(&cmd, height_correction);
                        if !throttles.iter().any(|t| t.is_nan()) {
                            for (motor, &throttle) in drone.motors.iter_mut().zip(throttles.iter())
                            {
                                motor.target_throttle = throttle.clamp(0.0, 1.0);
                            }
                        }
                    }
                    DroneCommand::Arm | DroneCommand::Disarm => {
                        for motor in &mut drone.motors {
                            motor.target_throttle = 0.0;
                        }
                    }
                }
            }
        }
    }
}

use super::drone::Drone;
use crate::types::DroneState;
use bevy::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Resource)]
pub struct SimStateSync(pub Arc<Mutex<DroneState>>);

pub fn update_state_sync(query: Query<(&Transform, &Drone)>, state_sync: Res<SimStateSync>) {
    if let Ok((transform, drone)) = query.get_single() {
        if let Ok(mut state) = state_sync.0.try_lock() {
            let (roll, pitch, yaw) = transform.rotation.to_euler(EulerRot::XYZ);
            *state = DroneState {
                roll: roll.to_degrees(),
                pitch: pitch.to_degrees(),
                yaw: yaw.to_degrees(),
                thrust: (drone.motors.iter().map(|m| m.current_throttle).sum::<f32>() * 65535.0)
                    as u16,
                armed: drone.motors.iter().any(|m| m.current_throttle > 0.0),
                battery_voltage: 3.7, // Simulated battery voltage
            };
        }
    }
}

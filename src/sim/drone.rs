use crate::{sim::constants::*, types::RpytCommand};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct DroneMotor {
    pub current_throttle: f32,
    pub target_throttle: f32,
}

#[derive(Component)]
pub struct Drone {
    pub motors: Vec<DroneMotor>,
}

#[derive(Component)]
pub struct HeightController {
    pub target: f32,
    pub integral: f32,
    pub last_error: f32,
}

#[derive(Bundle)]
pub struct DroneBundle {
    drone: Drone,
    height_controller: HeightController,
    rigid_body: RigidBody,
    collider: Collider,
    velocity: Velocity,
    external_force: ExternalForce,
    damping: Damping,
    mass_properties: ColliderMassProperties,
    transform: Transform,
    global_transform: GlobalTransform,
}

impl Default for DroneBundle {
    fn default() -> Self {
        Self {
            drone: Drone {
                motors: vec![
                    DroneMotor {
                        current_throttle: 0.0,
                        target_throttle: 0.0,
                    },
                    DroneMotor {
                        current_throttle: 0.0,
                        target_throttle: 0.0,
                    },
                    DroneMotor {
                        current_throttle: 0.0,
                        target_throttle: 0.0,
                    },
                    DroneMotor {
                        current_throttle: 0.0,
                        target_throttle: 0.0,
                    },
                ],
            },
            height_controller: HeightController {
                target: 1.0,
                integral: 0.0,
                last_error: 0.0,
            },
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(0.05, 0.02, 0.05), // Simple box shape
            velocity: Velocity::zero(),
            external_force: ExternalForce::default(),
            damping: Damping {
                linear_damping: 0.1,
                angular_damping: 1.0,
            },
            mass_properties: ColliderMassProperties::Mass(DRONE_MASS),
            transform: Transform::from_xyz(0.0, 0.0, 0.0), // Start at ground
            global_transform: GlobalTransform::default(),
        }
    }
}

pub fn setup_drone(mut commands: Commands) {
    commands.spawn(DroneBundle::default());
}

pub fn apply_motor_forces(
    mut query: Query<(&mut Drone, &mut ExternalForce, &Transform, &Velocity)>,
) {
    for (mut drone, mut external_force, transform, velocity) in query.iter_mut() {
        let mut total_thrust = 0.0;

        // Calculate total thrust
        for motor in drone.motors.iter_mut() {
            motor.current_throttle = motor.target_throttle;
            let thrust = motor.current_throttle * MAX_THRUST_PER_MOTOR;
            total_thrust += thrust;
        }

        // Calculate net force (thrust - gravity)
        let gravity_force = GRAVITY * DRONE_MASS;
        let net_force = total_thrust - gravity_force;

        // Apply forces
        external_force.force = Vec3::new(0.0, net_force, 0.0);
        external_force.torque = Vec3::ZERO;

        println!(
            "Physics: Height={:.3}m Vel={:.3}m/s Force={:.3}N Damping={:.3}",
            transform.translation.y,
            velocity.linvel.y,
            external_force.force.y,
            velocity.linvel.y * -1.0 // Damping force
        );

        println!(
            "Forces: Thrust={:.4}N | Gravity={:.4}N | Net={:.4}N | Throttles=[{:.3}, {:.3}, {:.3}, {:.3}]",
            total_thrust,
            gravity_force,
            net_force,
            drone.motors[0].current_throttle,
            drone.motors[1].current_throttle,
            drone.motors[2].current_throttle,
            drone.motors[3].current_throttle,
        );
    }
}

pub fn calculate_motor_throttles(cmd: &RpytCommand, height_correction: f32) -> [f32; 4] {
    let height_correction = height_correction.clamp(-0.3, 0.3);

    // Map the input thrust differently
    let thrust_normalized = (cmd.thrust as f32) / 65535.0; // 0 to 1
    let base_thrust = thrust_normalized + height_correction;
    let thrust = base_thrust.clamp(0.0, 1.0);

    // For now, ignore roll/pitch/yaw
    [thrust, thrust, thrust, thrust]
}

impl Default for HeightController {
    fn default() -> Self {
        Self {
            target: 1.0, // Target height in meters
            integral: 0.0,
            last_error: 0.0,
        }
    }
}

pub fn height_control(mut query: Query<(&HeightController, &Transform, &Velocity, &mut Drone)>) {
    for (controller, transform, velocity, mut drone) in query.iter_mut() {
        let error = controller.target - transform.translation.y;

        // Simple PD controller
        let correction = error * HEIGHT_P_GAIN + (-velocity.linvel.y * HEIGHT_D_GAIN);
        let throttle = (BASE_THROTTLE + correction).clamp(0.0, 1.0);

        // Apply same throttle to all motors
        for motor in drone.motors.iter_mut() {
            motor.target_throttle = throttle;
        }

        println!(
            "Control: Height={:.3}m Target={:.1}m Error={:.3}m Velocity={:.3}m/s Throttle={:.3}",
            transform.translation.y, controller.target, error, velocity.linvel.y, throttle
        );
    }
}

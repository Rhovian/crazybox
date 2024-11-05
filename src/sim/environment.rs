use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup_environment(mut commands: Commands) {
    // Add camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.0, -2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Add light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Add ground plane
    commands.spawn((
        Collider::cuboid(50.0, 0.1, 50.0),
        TransformBundle::from(Transform::from_xyz(0.0, -0.1, 0.0)),
    ));
}

use bevy::{app::App, DefaultPlugins};
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use crazybox::sim::world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            // Optional debug rendering
            // RapierDebugRenderPlugin::default(),
            WorldPlugin,
        ))
        .run();
}

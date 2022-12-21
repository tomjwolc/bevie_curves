use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_prototype_debug_lines::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bevy-ier curves".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(DebugLinesPlugin::with_depth_test(true))

        .add_startup_system(setup)

        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());


}
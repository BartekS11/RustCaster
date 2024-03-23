use bevy::prelude::*;

use crate::camera::setup_camera;

pub struct WorldPlugin;

fn world_startup_system(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.9,
    });
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1280.0, 720.0).into(),
                title: "RustCaster".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, world_startup_system)
        .add_systems(Startup, setup_camera)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)));
    }
}

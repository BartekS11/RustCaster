use bevy::prelude::*;
mod fps_counter;
mod world;
mod camera;

use camera::setup_camera;
use fps_counter::FpsCounter;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_plugins(FpsCounter)
        .add_plugins(WorldPlugin)
        .run();
}
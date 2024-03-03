#[warn(non_snake_case)]
use bevy::prelude::*;
mod fps_counter;
mod world;
mod camera;

use fps_counter::FpsCounter;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(WorldPlugin)
        .add_plugins(FpsCounter)
        .run();
}

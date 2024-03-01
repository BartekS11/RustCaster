use bevy::prelude::*;
mod fps_counter;

use fps_counter::FpsCounter;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FpsCounter)
        .run();
}
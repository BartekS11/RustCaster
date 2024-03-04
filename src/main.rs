#[warn(non_snake_case)]

use bevy::prelude::*;
mod debug_ui;
mod world;
mod camera;

use debug_ui::DebugUI;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(WorldPlugin)
        .add_plugins(DebugUI)
        .run();
}

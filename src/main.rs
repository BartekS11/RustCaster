#![allow(non_snake_case)]

use bevy::prelude::*;
mod camera;
mod debug_ui;
mod map;
mod player;
mod raycasting;
mod world;

use debug_ui::DebugUI;
use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(WorldPlugin)
        .add_plugins(DebugUI)
        .add_plugins(PlayerPlugin)
        .run();
}

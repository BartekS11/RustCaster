#![allow(non_snake_case)]

use bevy::prelude::*;
mod camera;
mod common_utils;
mod debug_ui;
mod main_ui;
mod map;
mod player;
mod raycasting;
mod world;

use debug_ui::DebugUI;
// use main_ui::MainUI;
use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(WorldPlugin)
        .add_plugins(DebugUI)
        // .add_plugins(MainUI)
        .add_plugins(PlayerPlugin)
        .run();
}

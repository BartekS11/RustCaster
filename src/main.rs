#![allow(non_snake_case)]

use bevy::prelude::*;
mod camera;
mod debug_ui;
mod map;
mod player;
mod world;
mod raycasting;

use debug_ui::DebugUI;
use map::MapPlugin;
use player::PlayerPlugin;
use raycasting::RaycastingPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(WorldPlugin)
        .add_plugins(DebugUI)
        .add_plugins(MapPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(RaycastingPlugin)
        .run();
}

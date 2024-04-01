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

use common_utils::{game::GameState, GameStatePlugin};
use debug_ui::DebugUI;
use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(GameStatePlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(DebugUI {
            state: GameState::Debug,
        })
        .add_plugins(PlayerPlugin)
        .run();
}

use bevy::prelude::*;

use self::{
    crosshair::{init, spawn_crosshair},
    game::{toggle_debug_release_mode, GameState},
};

pub mod crosshair;
pub mod game;

pub struct CrosshairPlugin;
pub struct GameStatePlugin;

impl Plugin for CrosshairPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_systems(Update, spawn_crosshair);
    }
}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, toggle_debug_release_mode);
    }
}

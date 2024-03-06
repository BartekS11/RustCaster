use bevy::prelude::*;
use self::player_systems::spawn_player;

pub mod player_component;
pub mod player_ray;
pub mod player_constants;
pub mod player_systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

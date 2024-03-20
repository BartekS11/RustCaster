use self::player_systems::{player_movement, player_spawn, start_raycast_for_player};
use bevy::prelude::*;

pub mod player_component;
pub mod player_constants;
pub mod player_ray;
pub mod player_systems;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_spawn)
            .add_systems(Update, (player_movement, start_raycast_for_player));
    }
}

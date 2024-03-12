use self::{
    player_component::Player,
    player_systems::{player_movement, player_spawn, start_raycast_for_player},
};
use bevy::prelude::*;

pub mod player_component;
pub mod player_constants;
pub mod player_ray;
pub mod player_systems;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // .insert_resource(Player::new())
            .add_systems(Startup, player_spawn)
            .add_systems(Update, player_movement)
            .add_systems(Update, start_raycast_for_player);
    }
}

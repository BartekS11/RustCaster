use bevy::prelude::*;

use crate::raycasting::rays::get_player_view;

#[derive(Component, Resource)]
pub struct Player {
    pub rotation: f32,
    pub health_points: u32,
    pub velocity: Vec2,
    pub is_collision_on: bool,
}

impl Player {
    pub fn get_view(&self) -> [(i32, bool); 800] {
        get_player_view(self.velocity.x, self.velocity.y, self.rotation)
    }
}

use bevy::prelude::*;

use crate::raycasting::rays::get_player_view;

// use super::player_constants::PLAYER_STARTING_ROTATION;

#[derive(Component, Resource)]
pub struct Player {
    pub rotation: f32,
    pub health_points: u32,
    pub velocity: Vec3,
    pub is_collision_on: bool,
    // pub rays: Vec<PlayerRay>,
}

impl Player {
    pub fn get_view(&mut self) -> [i32; 1280] {
        get_player_view(self.velocity.x, self.velocity.y, self.rotation)
    }
    // pub fn new() -> Player {
    //     Player {
    //         rotation: PLAYER_STARTING_ROTATION,
    //         health_points: 100,
    //         velocity: default(),
    //         is_collision_on: true,
    //     }
    // }
}

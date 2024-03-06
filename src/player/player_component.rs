use bevy::prelude::*;
use super::player_ray::PlayerRay;

#[derive(Component)]
pub struct Player {
    pub rotation: f32,
    pub health_points: u32,
    pub velocity: Vec3,
    pub is_colliding: bool,
    pub rays: Vec<PlayerRay>,
}
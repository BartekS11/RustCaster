use bevy::prelude::*;

pub struct PlayerRay {
    pub start: Vec3,
    pub end: Vec3,
    pub distance: f32,
    pub rotation: f32,
    pub color: Color
}
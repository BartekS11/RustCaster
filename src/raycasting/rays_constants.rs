use std::f32::consts::PI;

// pub const RAYS_AMOUNT: f32 = 1280.0;
pub const RAYS_AMOUNT: f32 = 800.0;
pub const FOV: f32 = PI / 2.7;
pub const HALF_FOV: f32 = FOV / 2.0;
pub const RAY_ANGLE_INCREMENT: f32 = FOV / RAYS_AMOUNT;
pub const WALL_HEIGHT: f32 = 500.0;

// use bevy::prelude::*;

// use crate::player::player_ray::RayForPlayer;

// // use super::rays_constants::{HALF_FOV, PRECISION, RAYS_AMOUNT, RAY_ANGLE_INCREMENT};

// pub(crate) fn raycaster(x: f32, y: f32, player_angle: f32) -> [RayForPlayer; RAYS_AMOUNT] {
//     let mut ray_angle: f32 = player_angle - HALF_FOV;

//     let mut rays = [RayForPlayer::default(); RAYS_AMOUNT];

//     for (_count, ray) in rays.iter_mut().enumerate() {
//         let mut ray_x: f32 = x;
//         let mut ray_y: f32 = y;

//         let ray_cos = ray_angle.to_radians().cos() / PRECISION;
//         let ray_sin = ray_angle.to_radians().sin() / PRECISION;

//         // for _i in 0..RAY_MAX {
//         //     if wall_point(ray_x, ray_y) {
//         //         break;
//         //     }
//         //     ray_x += ray_cos;
//         //     ray_y += ray_sin;
//         // }

//         *ray = RayForPlayer {
//             x: ray_x,
//             y: ray_y,
//             distance: (ray_angle - player_angle).to_radians().cos()
//                 * get_distance(x - ray_x, y - ray_y),
//         };

//         ray_angle += RAY_ANGLE_INCREMENT;
//     }

//     rays
// }

// fn get_distance(x: f32, y: f32) -> f32 {
//     ((x * x) + (y * y)).sqrt()
// }

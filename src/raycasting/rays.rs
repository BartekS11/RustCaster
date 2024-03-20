use std::f32::consts::{FRAC_2_PI, PI};

use super::rays_constants::{HALF_FOV, RAY_ANGLE_INCREMENT, WALL_HEIGHT};
use crate::map::map_systems::map_collision_points;

fn get_distance(x: f32, y: f32) -> f32 {
    ((x * x) + (y * y)).sqrt()
}

fn raycast_horizontal_intersection(
    player_velocity_x: f32,
    player_velocity_y: f32,
    player_rotation: f32,
) -> f32 {
    // Calculate up angle
    let up_angle = ((player_rotation / PI).floor() % 2.0).abs() != 0.0;
    let intersect_first_y = if up_angle {
        (player_velocity_y).ceil() - player_velocity_y
    } else {
        (player_velocity_y).floor() - player_velocity_y
    };

    let intersect_first_x = -intersect_first_y / player_rotation.tan();

    let dy = if up_angle { 1.0 } else { -1.0 };
    let dx = -dy / (player_rotation).tan();

    // Ray distance from player
    let mut next_ray_from_player_x = intersect_first_x;
    let mut next_ray_from_player_y = intersect_first_y;

    // Loop that extends ray until it hits wall
    for _ in 0..256 {
        let current_x = next_ray_from_player_x + player_velocity_x;
        let current_y = if up_angle {
            next_ray_from_player_y + player_velocity_y
        } else {
            next_ray_from_player_y + player_velocity_y - 1.0
        };

        // Break loop when collision with wall will happen
        if map_collision_points(current_x, current_y) {
            break;
        }
        next_ray_from_player_x += dx;
        next_ray_from_player_y += dy;
    }
    get_distance(next_ray_from_player_x, next_ray_from_player_y)
}

fn raycast_vertical_intersection(
    player_velocity_x: f32,
    player_velocity_y: f32,
    player_rotation: f32,
) -> f32 {
    let right = (((player_rotation - FRAC_2_PI) / PI).floor() % 2.0).abs() != 0.0;

    let intersect_first_x = if right {
        (player_velocity_x).ceil() - player_velocity_x
    } else {
        (player_velocity_x).floor() - player_velocity_x
    };
    let intersect_first_y = -(player_rotation).tan() * intersect_first_x;

    let dx = if right { 1.0 } else { -1.0 };
    let dy = dx * -(player_rotation).tan();

    // Ray distance from player
    let mut next_ray_from_player_x = intersect_first_x;
    let mut next_ray_from_player_y = intersect_first_y;

    // Loop that extends ray until it hits wall
    for _ in 0..256 {
        let current_x = if right {
            next_ray_from_player_x + player_velocity_x
        } else {
            next_ray_from_player_x + player_velocity_x - 1.0
        };
        let current_y = next_ray_from_player_y + player_velocity_y;

        // Break loop when collision with wall will happen
        if map_collision_points(current_x, current_y) {
            break;
        }
        next_ray_from_player_x += dx;
        next_ray_from_player_y += dy;
    }
    get_distance(next_ray_from_player_x, next_ray_from_player_y)
}

pub fn get_player_view(
    player_velocity_x: f32,
    player_velocity_y: f32,
    player_rotation: f32,
) -> [i32; 1280] {
    let start_angle = player_rotation + HALF_FOV;

    let mut walls = [0; 1280];

    for (idx, wall) in walls.iter_mut().enumerate() {
        let angle = start_angle - idx as f32 * RAY_ANGLE_INCREMENT;

        let h_dist = raycast_horizontal_intersection(player_velocity_x, player_velocity_y, angle);
        let v_dist = raycast_vertical_intersection(player_velocity_x, player_velocity_y, angle);

        *wall = (WALL_HEIGHT / f32::min(h_dist, v_dist)) as i32;
    }
    walls
}

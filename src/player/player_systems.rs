use crate::map::map_systems::map_collision_points;

use super::{
    player_component::Player,
    player_constants::{
        PLAYER_ROTATING_SPEED, PLAYER_SPEED, PLAYER_STARTING_POSITION, PLAYER_STARTING_ROTATION,
        WALL_COLOR, WALL_COLOR_SHADOW,
    },
};
use bevy::prelude::*;

pub fn player_spawn(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                PLAYER_STARTING_POSITION.x,
                PLAYER_STARTING_POSITION.y,
                PLAYER_STARTING_ROTATION,
            ),
            ..default()
        },
        Player {
            rotation: PLAYER_STARTING_ROTATION,
            health_points: 101,
            velocity: PLAYER_STARTING_POSITION,
            is_collision_on: true,
        },
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Player, With<Player>>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        let previous_velocity = (player.velocity.x, player.velocity.y);

        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            player.velocity.x += (player.rotation).cos() * PLAYER_SPEED * time.delta_seconds();
            player.velocity.y += -(player.rotation).sin() * PLAYER_SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            player.velocity.x -= (player.rotation).cos() * PLAYER_SPEED * time.delta_seconds();
            player.velocity.y -= -(player.rotation).sin() * PLAYER_SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            player.rotation += PLAYER_ROTATING_SPEED;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            player.rotation -= PLAYER_ROTATING_SPEED;
        }
        if player.is_collision_on && map_collision_points(player.velocity.x, player.velocity.y) {
            (player.velocity.x, player.velocity.y) = previous_velocity;
        }

        println!(
            "x: {}, y: {}, rot: {}",
            player.velocity.x, player.velocity.y, player.rotation
        );
    }
}

pub fn start_raycast_for_player(mut gizmos: Gizmos, player_query: Query<&Player, With<Player>>) {
    if let Ok(player) = player_query.get_single() {
        for (ray, wall_height) in player.get_view().iter().enumerate() {
            let (height, shadow) = wall_height;
            let mut colorWall: Color = WALL_COLOR;

            if !*shadow {
                colorWall = WALL_COLOR_SHADOW;
            }
            let y_top = (200 - (height / 2)) as f32;
            gizmos.ray_2d(
                Vec2::new(ray as f32, y_top),
                Vec2::new(0.0, *height as f32),
                colorWall,
            );
        }
    }
}

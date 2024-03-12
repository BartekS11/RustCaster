use super::{
    player_component::Player,
    player_constants::{
        PLAYER_ROTATING_SPEED, PLAYER_SPEED, PLAYER_STARTING_POSITION, PLAYER_STARTING_ROTATION,
    },
};
use bevy::prelude::*;

pub fn player_spawn(mut commands: Commands, assets_serv: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                PLAYER_STARTING_POSITION.x,
                PLAYER_STARTING_POSITION.y,
                0.0,
            ),
            texture: assets_serv.load("player.png"),
            ..default()
        },
        Player {
            rotation: PLAYER_STARTING_ROTATION,
            health_points: 100,
            velocity: default(),
            is_collision_on: true,
            // rays: vec![],
        },
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
) {
    let mut dir = Vec3::default();
    if let Ok((mut transform, mut player)) = player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            dir.x = player.rotation.to_radians().sin();
            dir.y = player.rotation.to_radians().cos();
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            dir.y = -player.rotation.to_radians().sin();
            dir.x = -player.rotation.to_radians().cos();
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            player.rotation += PLAYER_ROTATING_SPEED;
            transform.rotate_z(PLAYER_ROTATING_SPEED.to_radians());
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            player.rotation -= PLAYER_ROTATING_SPEED;
            transform.rotate_z(-PLAYER_ROTATING_SPEED.to_radians());
        }

        player.rotation = adjust_rotation(player.rotation);

        if dir.length() > 0.0 {
            dir = dir.normalize();
        }

        transform.translation += dir * PLAYER_SPEED * time.delta_seconds();
        player.velocity = dir;
    }
}

pub fn adjust_rotation(rotation: f32) -> f32 {
    let new_rotation;
    if rotation >= 360.0 {
        new_rotation = (360.0 - rotation).abs();
    } else if rotation < 0.0 {
        new_rotation = 360.0 - rotation.abs();
    } else {
        new_rotation = rotation;
    }

    new_rotation
}

pub fn start_raycast_for_player(mut player: ResMut<Player>) {
    player.get_view();
}

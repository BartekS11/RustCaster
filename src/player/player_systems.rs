use bevy::prelude::*;

use super::player_constants::TILESIZE;

#[derive(Component)]
pub struct Ground {}

pub fn spawn_player(
    mut commands: Commands,
    window: Query<&Window>,
    assets_serv: Res<AssetServer>
) {
    let window = window.single();
    let width = window.resolution.width();
    let height = window.resolution.height();

    let mut walls = vec![];

    for pos_x in 0..(width / TILESIZE).ceil() as i32 {
        for pos_y in 0..(height / TILESIZE).ceil() as i32 {
            walls.push((
                ((pos_x as f32) * TILESIZE) + (TILESIZE), 
                ((pos_y as f32) * TILESIZE) + (TILESIZE)
            ));
        }
    }

    for pos in walls {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(pos.0, pos.1, 0.0),
                texture: assets_serv.load("ground.png"),
                ..default()
            },
            Ground {}
        ));
    }
}
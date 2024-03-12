use bevy::prelude::*;

use crate::player::player_constants::TILESIZE;

const MAP: [u16; 8] = [
    0b1111111111111111,
    0b1000001010000101,
    0b1011100000110101,
    0b1000111010010001,
    0b1010001011110111,
    0b1011101001100001,
    0b1000100000001101,
    0b1111111111111111,
];

pub fn map_collision_points(x: f32, y: f32) -> bool {
    match MAP.get(y as usize) {
        Some(line) => (line & (0b1 << x as usize)) != 0,
        _ => true,
    }
}

#[derive(Component)]
pub struct Ground {}

pub fn map_spawn(mut commands: Commands, window: Query<&Window>, assets_serv: Res<AssetServer>) {
    let window = window.single();
    let width = window.resolution.width();
    let height = window.resolution.height();

    let mut walls = vec![];

    for pos_x in 0..(width / TILESIZE).ceil() as i32 {
        for pos_y in 0..(height / TILESIZE).ceil() as i32 {
            walls.push((
                ((pos_x as f32) * TILESIZE) + (TILESIZE),
                ((pos_y as f32) * TILESIZE) + (TILESIZE),
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
            Ground {},
        ));
    }
}

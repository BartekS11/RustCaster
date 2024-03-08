use bevy::prelude::*;

// use crate::player::player_component::Player;
use crate::player::player_constants::TILESIZE;

// #[derive(Debug)]
// pub struct Map
// {
// 	pub width: i32,
// 	pub height: i32,
// 	pub map: Vec<u8>,
// 	pub player: Player
// }

#[derive(Component)]
pub struct Ground {}

// fn parse_map() {
//     let map = vec![
//         1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
//         1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
//         1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
//         1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
//         1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
//         1, 0, 0, 1, 0, 1, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
//         1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
//         1, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
//         1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
//         1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
//         1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
//         1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
//         1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
//         1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
//         1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
//         1, 1, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
//     ];

// }

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

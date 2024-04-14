use bevy::prelude::*;
use std::collections::HashMap;

use super::map_component::CustomAssetHandler;

const TILESIZE: f32 = 41.0;
const TEXTURE_NAMES: [&str; 4] = ["wall-stone", "floor-tile", "door", "ceiling-tile"];
const SHADOW_DISTANCE: f32 = 4.0;

pub fn color_distance(color: Color, distance: f32) -> Color {
    let light_color = color.with_a(1.);
    // If distance not far enough just keep color
    if distance < SHADOW_DISTANCE {
        return light_color;
    }

    light_color.with_a(1. / (distance - SHADOW_DISTANCE))
}

#[derive(Component)]
pub struct Ground {}

// const MAP: [[u8; 16]; 16] = [
//     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
//     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
//     [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
//     [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
//     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
//     [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
//     [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
//     [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
//     [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
//     [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
//     [1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1],
//     [1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1],
//     [1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1],
//     [1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1],
//     [1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1],
//     [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
// ];
// const IMAGE_TEXTURE_LOAD: [Handle<Image>; 4] = [

// ];
const MAP: [[u8; 16]; 16] = [
    [1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
    [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 3],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 3],
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 3],
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 3],
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 1, 0, 0, 2, 2, 0, 2, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1],
    [1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1],
    [1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1],
    [1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1],
];

pub fn map_collision_points(x: f32, y: f32) -> bool {
    match MAP.get(y as usize) {
        Some(row) => match row.get(x as usize) {
            Some(wall) => *wall != 0,
            None => true,
        },
        None => true,
    }
}

pub fn load_map_assets(mut commands: Commands, assets_serv: Res<AssetServer>) {
    let mut textures = HashMap::new();
    for name in TEXTURE_NAMES {
        let handle = assets_serv.load(format!("{}.png", name));
        textures.insert(name, handle);
    }
    commands.insert_resource(CustomAssetHandler { texture: textures })
}

pub fn map_spawn(mut commands: Commands, window: Query<&Window>, assets: Res<CustomAssetHandler>) {
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
                sprite: Sprite {
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    rect: Some(Rect {
                        min: Vec2::new(0., 0.),
                        max: Vec2::new(1., 16.0 - 1.),
                }),
                     ..default()
                },
                transform: Transform::from_xyz(pos.0, pos.1, 0.0),
                texture: assets.texture["wall-stone"].clone(),
                ..default()
            },
            Ground {},
        ));
    }
}
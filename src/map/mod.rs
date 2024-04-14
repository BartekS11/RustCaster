use bevy::prelude::*;

use self::map_systems::{load_map_assets, map_spawn};

pub mod map_systems;
pub mod map_component;


pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, load_map_assets.before(map_spawn))
            .add_systems(PreStartup, map_spawn);
    }
}
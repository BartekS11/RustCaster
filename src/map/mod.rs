use bevy::prelude::*;

use self::map_systems::map_spawn;

mod map_systems;
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, map_spawn);
    }
}

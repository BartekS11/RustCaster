use bevy::prelude::*;

use self::crosshair::{init, spawn_crosshair};

pub mod crosshair;

pub struct CrosshairPlugin;

impl Plugin for CrosshairPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_systems(Update, spawn_crosshair);
    }
}

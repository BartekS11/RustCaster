use bevy::prelude::*;
mod debug_ui;
mod world;
mod camera;
mod player;

use debug_ui::DebugUI;
use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(WorldPlugin)
        .add_plugins(DebugUI)
        .add_plugins(PlayerPlugin)
        .run();
}

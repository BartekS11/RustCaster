use bevy::{prelude::*, window::*};

use crate::common_utils::CrosshairPlugin;

pub struct MainUI;

impl Plugin for MainUI {
    fn build(&self, app: &mut App) {
        app.add_plugins(CrosshairPlugin)
            .add_systems(Update, close_on_esc);
    }
}

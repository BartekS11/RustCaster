use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Resource)]
pub struct CustomAssetHandler {
    pub texture: HashMap<&'static str, Handle<Image>>
}
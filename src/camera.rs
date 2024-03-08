use bevy::prelude::*;

#[derive(Component)]
pub struct MyCameraMarker;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::default(),
            ..default()
        },
        MyCameraMarker,
    ));
}

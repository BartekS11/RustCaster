use bevy::prelude::*;

#[derive(Component)]
pub struct MyCameraMarker;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(1280.0 / 2.0, 0.0, 0.0),
            // transform: Transform::default(),
            ..default()
        },
        MyCameraMarker,
    ));
}

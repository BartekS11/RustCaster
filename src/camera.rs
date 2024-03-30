use bevy::prelude::*;

#[derive(Component)]
pub struct MyCameraMarker;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(400.0, 200.0, 0.0),
            ..default()
        },
        MyCameraMarker,
    ));
}

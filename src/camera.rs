use bevy::prelude::*;

#[derive(Component)]
pub struct MyCameraMarker;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(600.0, -300.0, 0.0),
            ..default()
        },
        MyCameraMarker,
    ));
}

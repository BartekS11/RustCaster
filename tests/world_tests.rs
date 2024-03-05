use bevy::app::{App, Update};
use bevy::prelude::*;


fn spawn_mock_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::new_with_far(1.0));
}

#[test]
fn did_app() {
    let mut app = App::new();

    app.add_systems(Update, spawn_mock_camera);

    app.update();

    assert_eq!(app.world.query::<&Camera2d>().iter(&app.world).len(), 1);
}
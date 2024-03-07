use bevy::prelude::*;

#[derive(Component, Default)]
struct PlayerMock {
    pub position: f32,
    pub health_points: u32,
}

fn spawn_mocked_player(mut commands: Commands, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        commands.spawn(PlayerMock::default());
    }
}

#[test]
fn did_player_spawned_correctly() {
    let mut app = App::new();

    app.add_systems(Update, spawn_mocked_player);

    let mut input = ButtonInput::<KeyCode>::default();
    input.press(KeyCode::Space);
    app.insert_resource(input);

    app.update();

    assert_eq!(app.world.query::<&PlayerMock>().iter(&app.world).len(), 1);
    app.world.resource_mut::<ButtonInput<KeyCode>>().clear();

    app.update();

    assert_eq!(app.world.query::<&PlayerMock>().iter(&app.world).len(), 1);
}

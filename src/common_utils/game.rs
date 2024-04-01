use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Debug,
    Release,
}

pub fn toggle_debug_release_mode(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Backspace) {
        if simulation_state.get() == &GameState::Debug {
            commands.insert_resource(NextState(Some(GameState::Release)));
            println!("Release toggled");
        } else if simulation_state.get() == &GameState::Release {
            commands.insert_resource(NextState(Some(GameState::Debug)));
            println!("Debug toggled");
        }
    }
}

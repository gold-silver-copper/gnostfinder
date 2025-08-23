use crate::*;

pub enum InputState {
    None,
    Up,
    Down,
    Left,
    Right,
    Select,
    Back,
}

// This function implements `Plugin`, along with every other `fn(&mut App)`.
pub fn input_systems_plugin(app: &mut App) {
    app.add_systems(PreUpdate, input_handler);
}

/// Handles input when in the menu.
fn input_handler(keys: Res<ButtonInput<KeyCode>>, mut game_state: ResMut<GameState>) {
    game_state.input_state = InputState::None;
    if keys.just_pressed(KeyCode::ArrowDown) {
        game_state.input_state = InputState::Down;
    }
    if keys.just_pressed(KeyCode::ArrowUp) {
        game_state.input_state = InputState::Up;
    }
    if keys.just_pressed(KeyCode::Enter) {
        game_state.input_state = InputState::Select;
    }
    if keys.just_pressed(KeyCode::KeyQ) {
        game_state.input_state = InputState::Back;
    }
    if keys.just_pressed(KeyCode::ArrowLeft) {
        game_state.input_state = InputState::Left;
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        game_state.input_state = InputState::Right;
    }
}

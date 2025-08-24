use crate::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LastInput {
    None,
    Up,
    Down,
    Left,
    Right,
    Select,
    Back,
    Opt1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputState {
    Main,
    Movement,
}

impl GameState {
    fn handle_main_input(&mut self) {
        match self.last_input {
            LastInput::Down => (),
            LastInput::Up => (),
            LastInput::Select => (),
            LastInput::Back => {
                panic!("aaa");
            }
            LastInput::Opt1 => self.input_state = InputState::Movement,
            _ => {}
        }
    }
    fn handle_movement_input(&mut self) {
        match self.last_input {
            LastInput::Down => (),
            LastInput::Up => (),
            LastInput::Select => (),
            LastInput::Back => self.input_state = InputState::Main,
            LastInput::Opt1 => (),
            _ => {}
        }
    }

    pub fn input_handler(&mut self) {
        match self.input_state {
            InputState::Main => {
                self.handle_main_input();
            }
            InputState::Movement => {
                self.handle_movement_input();
            }
        }
    }
}

// This function implements `Plugin`, along with every other `fn(&mut App)`.
pub fn input_systems_plugin(app: &mut App) {
    app.add_systems(PreUpdate, input_handler);
}

/// Handles input when in the menu.
fn input_handler(keys: Res<ButtonInput<KeyCode>>, mut game_state: ResMut<GameState>) {
    game_state.last_input = LastInput::None;
    if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        game_state.last_input = LastInput::Down;
    }
    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        game_state.last_input = LastInput::Up;
    }
    if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
        game_state.last_input = LastInput::Left;
    }
    if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD) {
        game_state.last_input = LastInput::Right;
    }
    if keys.just_pressed(KeyCode::Enter) {
        game_state.last_input = LastInput::Select;
    }
    if keys.just_pressed(KeyCode::KeyQ) {
        game_state.last_input = LastInput::Back;
    }
    if keys.just_pressed(KeyCode::KeyI) {
        game_state.last_input = LastInput::Opt1;
    }
}

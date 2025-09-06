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
    fn handle_main_input(&mut self, my_input: LastInput) {
        match my_input {
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
    fn handle_movement_input(&mut self, my_input: LastInput) {
        match my_input {
            LastInput::Down => (),
            LastInput::Up => (),
            LastInput::Select => (),
            LastInput::Back => self.input_state = InputState::Main,
            LastInput::Opt1 => (),
            _ => {}
        }
    }

    pub fn input_handler(&mut self, my_input: LastInput) {
        match self.input_state {
            InputState::Main => {
                self.handle_main_input(my_input);
            }
            InputState::Movement => {
                self.handle_movement_input(my_input);
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
    let mut my_input = LastInput::None;
    if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        my_input = LastInput::Down;
    }
    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        my_input = LastInput::Up;
    }
    if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
        my_input = LastInput::Left;
    }
    if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD) {
        my_input = LastInput::Right;
    }
    if keys.just_pressed(KeyCode::Enter) {
        my_input = LastInput::Select;
    }
    if keys.just_pressed(KeyCode::KeyQ) {
        my_input = LastInput::Back;
    }
    if keys.just_pressed(KeyCode::KeyI) {
        my_input = LastInput::Opt1;
    }
    game_state.input_handler(my_input);
}

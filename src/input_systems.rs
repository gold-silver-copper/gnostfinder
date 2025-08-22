use crate::*;

/// Handles input when in the menu.
fn menu_input(
    input_state: Res<InputState>,
    mut menu: ResMut<MenuState>,
    mut app_exit: EventWriter<AppExit>,
) {
    match *input_state {
        InputState::Down => {
            menu.selected = (menu.selected + 1) % menu.options.len();
        }
        InputState::Up => {
            if menu.selected == 0 {
                menu.selected = menu.options.len() - 1;
            } else {
                menu.selected -= 1;
            }
        }
        InputState::Select => menu.choice = Some(menu.selected),
        InputState::Back => {
            app_exit.write_default();
        }
        _ => {}
    }
}

/// Handles input when in the menu.
fn input_handler(keys: Res<ButtonInput<KeyCode>>, mut input_state: ResMut<InputState>) {
    *input_state = InputState::None;
    if keys.just_pressed(KeyCode::ArrowDown) {
        *input_state = InputState::Down;
    }
    if keys.just_pressed(KeyCode::ArrowUp) {
        *input_state = InputState::Up;
    }
    if keys.just_pressed(KeyCode::Enter) {
        *input_state = InputState::Select;
    }
    if keys.just_pressed(KeyCode::KeyQ) {
        *input_state = InputState::Back;
    }
    if keys.just_pressed(KeyCode::ArrowLeft) {
        *input_state = InputState::Left;
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        *input_state = InputState::Right;
    }
}

// This function implements `Plugin`, along with every other `fn(&mut App)`.
pub fn input_systems_plugin(app: &mut App) {
    app.add_systems(PreUpdate, input_handler)
        .add_systems(Update, menu_input.run_if(in_state(GameState::MainMenu)));
}

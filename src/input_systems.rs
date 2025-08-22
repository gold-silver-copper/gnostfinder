use crate::*;

/// Handles global input (works in any screen).
fn global_input(keys: Res<ButtonInput<KeyCode>>, mut app_exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::KeyQ) {
        app_exit.write_default();
    }
}

/// Handles input when in the menu.
fn menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut menu: ResMut<MenuState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit: EventWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::ArrowDown) {
        menu.selected = (menu.selected + 1) % menu.options.len();
    }
    if keys.just_pressed(KeyCode::ArrowUp) {
        if menu.selected == 0 {
            menu.selected = menu.options.len() - 1;
        } else {
            menu.selected -= 1;
        }
    }

    //app_exit.write_default(),
    if keys.just_pressed(KeyCode::Enter) {
        match menu.selected {
            0 => next_state.set(GameState::NewGame),
            1 => next_state.set(GameState::LoadGame),
            2 => next_state.set(GameState::Settings),
            3 => next_state.set(GameState::Exiting),
            _ => {}
        }
    }
}

// This function implements `Plugin`, along with every other `fn(&mut App)`.
pub fn input_systems_plugin(app: &mut App) {
    app.add_systems(Update, (menu_input).run_if(in_state(GameState::Menu)))
        .add_systems(PreUpdate, global_input);
}

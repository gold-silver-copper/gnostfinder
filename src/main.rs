use bevy::{app::AppExit, prelude::*};
use bevy_ratatui::{RatatuiContext, RatatuiPlugins};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};
mod draw_systems;
use draw_systems::*;

// Game states
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    NewGame,
    LoadGame,
    Settings,
    Exiting,
}

// Menu structure
#[derive(Resource)]
struct MenuState {
    options: Vec<&'static str>,
    selected: usize,
}

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        RatatuiPlugins {
            enable_input_forwarding: true,
            ..default()
        },
        draw_menus_plugin,
    ))
    .init_state::<GameState>()
    .insert_resource(MenuState {
        options: vec!["New Game", "Load Game", "Settings", "Exit"],
        selected: 0,
    })
    .add_systems(Update, (menu_input).run_if(in_state(GameState::Menu)))
    .add_systems(PreUpdate, global_input)
    .run();
}

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

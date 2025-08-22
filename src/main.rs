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
mod input_systems;
use input_systems::*;

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

    app.insert_resource(MenuState {
        options: vec!["New Game", "Load Game", "Settings", "Exit"],
        selected: 0,
    })
    .add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        RatatuiPlugins {
            enable_input_forwarding: true,
            ..default()
        },
        draw_menus_plugin,
        input_systems_plugin,
    ))
    .init_state::<GameState>()
    .run();
}

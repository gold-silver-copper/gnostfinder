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
    MainMenu,
    NewGame,
    LoadGame,
    Settings,
    Exiting,
}

// Menu structure
#[derive(Resource)]
struct MenuState {
    options: Vec<String>,
    selected: usize,
    choice: Option<usize>,
    back: bool,
}

impl MenuState {
    pub fn clear(&mut self) {
        self.options = Vec::new();
        self.selected = 0;
        self.choice = None;
        self.back = false;
    }
}

#[derive(Resource)]
pub enum InputState {
    None,
    Up,
    Down,
    Left,
    Right,
    Select,
    Back,
}

fn main() {
    let mut app = App::new();

    app.insert_resource(MenuState {
        options: Vec::new(),
        selected: 0,
        choice: None,
        back: false,
    })
    .insert_resource(InputState::None)
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

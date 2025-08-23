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
mod game_state;
use game_state::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(GameState {
        input_state: InputState::None,
        player_id: 0,
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
    .run();
}

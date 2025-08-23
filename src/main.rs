use bevy::prelude::*;
use bevy_ratatui::{RatatuiContext, RatatuiPlugins};
use ratatui::text::Line;
mod draw_systems;
use draw_systems::*;
mod input_systems;
use input_systems::*;
mod game_state;
use game_state::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(GameState::new())
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            RatatuiPlugins {
                enable_input_forwarding: true,
                ..default()
            },
            draw_menus_plugin,
            input_systems_plugin,
            game_state_plugin,
        ))
        .run();
}

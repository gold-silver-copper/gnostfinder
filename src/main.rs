use bevy::prelude::*;
use bevy_ratatui::{RatatuiContext, RatatuiPlugins};
use petgraph::Direction;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use ratatui::prelude::Alignment;
use ratatui::prelude::Rect;
use ratatui::text::Line;
mod draw_systems;
use draw_systems::*;
mod input_systems;
use input_systems::*;
mod game_state;
use game_state::*;
mod character_sheet;
use character_sheet::*;
mod thing;
mod thing_graph;
use std::fmt;
use thing_graph::*;
mod game_graph;
use game_graph::*;

use thing::*;
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

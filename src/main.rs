use core::panic;

use bevy::{app::AppExit, diagnostic::FrameCount, prelude::*};

use bevy_ratatui::{RatatuiContext, RatatuiPlugins};

use ratatui::text::Text;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        RatatuiPlugins {
            enable_input_forwarding: true,
            ..default()
        },
    ))
    .add_systems(Update, draw_system)
    .add_systems(PreUpdate, keyboard_input_system_windowed)
    .run();
}

fn keyboard_input_system_windowed(
    keys: Res<ButtonInput<KeyCode>>,
    mut app_exit: EventWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::KeyQ) {
        app_exit.write_default();
    }
    if keys.just_pressed(KeyCode::KeyP) {
        panic!("Panic!");
    }
}
fn draw_system(mut context: ResMut<RatatuiContext>) -> Result {
    context.draw(|frame| {
        let text = Text::raw("hello world\npress 'q' to quit");
        frame.render_widget(text, frame.area());
    })?;

    Ok(())
}

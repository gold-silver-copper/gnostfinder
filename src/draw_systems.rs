use crate::*;
use ratatui::widgets::{List, ListItem, ListState};

use ratatui::layout::{Constraint, Direction, Layout, Rect};

fn draw_system(mut context: ResMut<RatatuiContext>, menu: Res<MenuState>) {
    let _ = context.draw(|frame| {
        let area = frame.area();

        // Vertical layout: top padding, list, bottom padding
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),                        // top padding
                Constraint::Length(menu.options.len() as u16 + 2), // list height (+2 for borders)
                Constraint::Percentage(50),                        // bottom padding
            ])
            .split(area);

        // Horizontal layout: left padding, list, right padding
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25), // left padding
                Constraint::Percentage(50), // list width
                Constraint::Percentage(25), // right padding
            ])
            .split(vertical_chunks[1]); // put the list in the middle vertical chunk

        // Convert menu options into ListItems
        let items: Vec<ListItem> = menu.options.iter().map(|opt| ListItem::new(*opt)).collect();

        let mut state = ratatui::widgets::ListState::default();
        state.select(Some(menu.selected));

        let list = ratatui::widgets::List::new(items)
            .block(
                ratatui::widgets::Block::default()
                    .title("Main Menu")
                    .borders(ratatui::widgets::Borders::ALL),
            )
            .highlight_style(
                ratatui::style::Style::default()
                    .fg(ratatui::style::Color::Yellow)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, horizontal_chunks[1], &mut state);
    });
}

/// Screen: New Game
fn draw_new_game(mut context: ResMut<RatatuiContext>) {
    let _ = context.draw(|frame| {
        let area = frame.area();
        let text = Paragraph::new("New Game Screen\nPress 'q' to quit")
            .alignment(Alignment::Center)
            .block(Block::default().title("New Game").borders(Borders::ALL));
        frame.render_widget(text, area);
    });
}

/// Screen: Load Game
fn draw_load_game(mut context: ResMut<RatatuiContext>) {
    let _ = context.draw(|frame| {
        let area = frame.area();
        let text = Paragraph::new("Load Game Screen\nPress 'q' to quit")
            .alignment(Alignment::Center)
            .block(Block::default().title("Load Game").borders(Borders::ALL));
        frame.render_widget(text, area);
    });
}

/// Screen: Settings
fn draw_settings(mut context: ResMut<RatatuiContext>) {
    let _ = context.draw(|frame| {
        let area = frame.area();
        let text = Paragraph::new("Settings Screen\nPress 'q' to quit")
            .alignment(Alignment::Center)
            .block(Block::default().title("Settings").borders(Borders::ALL));
        frame.render_widget(text, area);
    });
}

// This function implements `Plugin`, along with every other `fn(&mut App)`.
pub fn draw_menus_plugin(app: &mut App) {
    app.add_systems(Update, (draw_system).run_if(in_state(GameState::Menu)))
        .add_systems(Update, draw_new_game.run_if(in_state(GameState::NewGame)))
        .add_systems(Update, draw_load_game.run_if(in_state(GameState::LoadGame)))
        .add_systems(Update, draw_settings.run_if(in_state(GameState::Settings)));
}

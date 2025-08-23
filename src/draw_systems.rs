use crate::*;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{List, ListItem, ListState};
use ratatui::{
    Frame,
    backend::CrosstermBackend,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Wrap},
};

// This function implements `Plugin`, along with every other `fn(&mut App)`.
pub fn draw_menus_plugin(app: &mut App) {
    app.add_systems(Update, (draw_rpg_screen));
}

/// Screen: RPG World
fn draw_rpg_screen(mut context: ResMut<RatatuiContext>) {
    let _ = context.draw(|frame| {
        let area = frame.area();

        // Split screen into main body + command bar
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(5),
                Constraint::Length(3), // command bar
            ])
            .split(area);

        // Split main body into sidebar + event feed
        let horizontal_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(10),
                Constraint::Length(30), // sidebar width
            ])
            .split(vertical_layout[0]);

        // Sidebar: Stats + Inventory
        let sidebar_text = vec![
            Line::from("Stats:"),
            Line::from("HP: 100/100"),
            Line::from("MP: 50/50"),
            Line::from("Level: 1"),
            Line::from("XP: 0"),
            Line::from(""),
            Line::from("Inventory:"),
            Line::from("- Sword"),
            Line::from("- Shield"),
            Line::from("- Potion x3"),
        ];
        let sidebar = Paragraph::new(sidebar_text)
            .block(Block::default().title("Character").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        frame.render_widget(sidebar, horizontal_layout[1]);

        // Event / text feed
        let event_text = vec![
            Line::from("You enter a dark forest. The trees whisper around you."),
            Line::from("A wild goblin appears!"),
            Line::from("You can attack, defend, or flee."),
        ];
        let events = Paragraph::new(event_text)
            .block(Block::default().title("Events").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        frame.render_widget(events, horizontal_layout[0]);

        // Command bar
        let commands =
            Paragraph::new("Commands: [a] Attack  [d] Defend  [f] Flee  [i] Inventory  [q] Quit")
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Actions"));
        frame.render_widget(commands, vertical_layout[1]);
    });
}

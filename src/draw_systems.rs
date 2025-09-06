use crate::*;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

// This function implements `Plugin`, along with every other `fn(&mut App)`.
pub fn draw_menus_plugin(app: &mut App) {
    app.add_systems(PostUpdate, get_frame_area);
}

fn draw_movement_screen(frame: &mut Frame<'_>, _game_state: &Res<GameState>) {
    let area = frame.area();

    // Vertical split: top / center / bottom
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40), // top padding
            Constraint::Length(7),      // center box height
            Constraint::Percentage(40), // bottom padding
        ])
        .split(area);

    // Horizontal split: left / center / right
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // left padding
            Constraint::Length(30),     // center box width
            Constraint::Percentage(30), // right padding
        ])
        .split(vertical_chunks[1]); // only split the middle vertical chunk

    // Movement options to display
    let movement_options = vec![
        Line::from("[W] Move Up"),
        Line::from("[A] Move Left"),
        Line::from("[S] Move Down"),
        Line::from("[D] Move Right"),
        Line::from("[Q] Cancel"),
    ];

    let movement_paragraph = Paragraph::new(movement_options)
        .block(Block::default().title("Movement").borders(Borders::ALL))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    // Render in the central chunk
    frame.render_widget(movement_paragraph, horizontal_chunks[1]);
}

/// Screen: RPG World
fn draw_rpg_screen(frame: &mut Frame<'_>, game_state: &Res<GameState>) {
    let area = frame.area();
    let bg = Block::default().style(Style::default().bg(Color::Black));
    frame.render_widget(bg, area);

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
    let mut sidebar_text = Vec::new();

    for y in 0..10 {
        let mut grid_string = String::new();
        for x in 0..10 {
            let current_coord = Coord { x, y };

            if let Some(_node_index) = game_state.coord_map.get(&current_coord) {
                grid_string.push('@'); // Draw something from the map
            } else {
                grid_string.push('.'); // Draw empty floor
            }
            grid_string.push(' '); // Add a space for better readability
        }
        sidebar_text.push(Line::from(grid_string));
    }
    let player_name = game_state.thing_graph[game_state.player_id].name();

    let sidebar = Paragraph::new(sidebar_text)
        .block(Block::default().title(player_name).borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(sidebar, horizontal_layout[1]);

    let desc_text = game_state.describe_player_location();

    // Event / text feed
    let event_text = vec![Line::from(desc_text), Line::from("BLAH BLAH")];
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
}

fn get_frame_area(mut context: ResMut<RatatuiContext>, game_state: Res<GameState>) {
    let _ = context.draw(|frame| {
        draw_rpg_screen(frame, &game_state);

        if game_state.input_state == InputState::Movement {
            draw_movement_screen(frame, &game_state);
        }
    });
}

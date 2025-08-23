use crate::*;
use ratatui::widgets::{List, ListItem, ListState};

use ratatui::layout::{Constraint, Direction, Layout, Rect};
fn draw_main_menu(
    mut context: ResMut<RatatuiContext>,
    mut menu: ResMut<MenuState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    menu.options = vec!["Worlds".into(), "Exit".into()];

    if menu.back {
        panic!("bye bye");
    }

    let _ = context.draw(|frame| {
        let area = frame.area();

        // Solid background
        frame.render_widget(
            Block::default().style(Style::default().bg(Color::Black)),
            area,
        );

        // Vertical layout: top (subtitle), menu, bottom (footer)
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),                     // top
                Constraint::Min(menu.options.len() as u16 + 4), // menu
                Constraint::Percentage(30),                     // bottom
            ])
            .split(area);

        // Horizontal layout for menu centering
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25), // left
                Constraint::Percentage(50), // middle
                Constraint::Percentage(25), // right
            ])
            .split(vertical_chunks[1]);

        // --- CLEAN TEXT SECTIONS ---
        let top_text = Paragraph::new("Welcome to the Adventure")
            .style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center);

        let bottom_text = Paragraph::new("Use ↑/↓ to navigate, Enter to select")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center);

        // --- MENU LIST ---
        let items: Vec<ListItem> = menu
            .options
            .iter()
            .map(|opt| ListItem::new(opt.clone()))
            .collect();

        let mut state = ratatui::widgets::ListState::default();
        state.select(Some(menu.selected));

        let list = ratatui::widgets::List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Main Menu ")
                    .style(Style::default().fg(Color::Gray)),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("› ");

        // --- RENDER ---
        frame.render_widget(top_text, vertical_chunks[0]);
        frame.render_widget(bottom_text, vertical_chunks[2]);
        frame.render_stateful_widget(list, horizontal_chunks[1], &mut state);
    });

    match menu.choice {
        Some(x) => match x {
            0 => next_state.set(GameState::WorldMenu),

            1 => next_state.set(GameState::Exiting),
            _ => {
                menu.choice = None;
            }
        },
        None => {}
    };
}

fn game_exit() {
    panic!("bye bye");
}

// This function implements `Plugin`, along with every other `fn(&mut App)`.
pub fn draw_menus_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (draw_main_menu).run_if(in_state(GameState::MainMenu)),
    )
    .add_systems(
        Update,
        (draw_new_game).run_if(in_state(GameState::WorldMenu)),
    )
    .add_systems(Update, (game_exit).run_if(in_state(GameState::Exiting)));
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

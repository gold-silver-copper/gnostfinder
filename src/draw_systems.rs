use crate::*;
use ratatui::widgets::{List, ListItem, ListState};

use ratatui::layout::{Constraint, Direction, Layout, Rect};

fn draw_main_menu(
    mut context: ResMut<RatatuiContext>,
    mut menu: ResMut<MenuState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    menu.options = vec![
        "New Game".into(),
        "Load Game".into(),
        "Settings".into(),
        "Exit".into(),
    ];

    if menu.back {
        next_state.set(GameState::Exiting);
    }
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
        let items: Vec<ListItem> = menu
            .options
            .iter()
            .map(|opt| ListItem::new(opt.clone()))
            .collect();

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

    match menu.choice {
        Some(x) => match x {
            0 => next_state.set(GameState::NewGame),
            1 => next_state.set(GameState::LoadGame),
            2 => next_state.set(GameState::Settings),
            3 => next_state.set(GameState::Exiting),
            _ => {}
        },
        None => {}
    };
    menu.choice = None;
}

fn draw_general_menu(
    mut context: ResMut<RatatuiContext>,
    mut menu: ResMut<MenuState>,
    mut next_state: ResMut<NextState<GameState>>,
    options_vec: Vec<String>,
) {
    menu.options = options_vec;

    if menu.back {
        panic!("bye bye");
    }
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
        let items: Vec<ListItem> = menu
            .options
            .iter()
            .map(|opt| ListItem::new(opt.clone()))
            .collect();

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

    match menu.choice {
        Some(x) => match x {
            0 => next_state.set(GameState::NewGame),
            1 => next_state.set(GameState::LoadGame),
            2 => next_state.set(GameState::Settings),
            3 => next_state.set(GameState::Exiting),
            _ => {}
        },
        None => {}
    };
    menu.choice = None;
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

fn game_exit() {
    panic!("bye bye");
}
// This function implements `Plugin`, along with every other `fn(&mut App)`.
pub fn draw_menus_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (draw_main_menu).run_if(in_state(GameState::MainMenu)),
    )
    .add_systems(Update, (draw_new_game).run_if(in_state(GameState::NewGame)))
    .add_systems(Update, (game_exit).run_if(in_state(GameState::Exiting)));
}

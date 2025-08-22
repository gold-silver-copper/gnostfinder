use crate::*;

/// Draws the main menu.
fn draw_system(mut context: ResMut<RatatuiContext>, menu: Res<MenuState>) {
    let _ = context.draw(|frame| {
        let area = frame.area();

        let mut lines: Vec<Line> = Vec::new();
        for (i, opt) in menu.options.iter().enumerate() {
            if i == menu.selected {
                lines.push(Line::from(Span::styled(
                    format!("> {opt} <"),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )));
            } else {
                lines.push(Line::from(Span::raw(format!("  {opt}  "))));
            }
        }

        let paragraph = Paragraph::new(Text::from(lines))
            .alignment(Alignment::Center)
            .block(Block::default().title("Main Menu").borders(Borders::ALL));

        frame.render_widget(paragraph, area);
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

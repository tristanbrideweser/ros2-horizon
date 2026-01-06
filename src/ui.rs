// src/ui.rs

use ratatui::{
    layout::{
        Constraint, 
        Direction,
        Layout
    }, 
    widgets::*,
    style::*, 
    Frame
};
use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Header
    let header = Block::default()
        .borders(Borders::ALL)
        .title("LazyROS")
        .style(Style::default().fg(Color::Cyan));
    frame.render_widget(header, chunks[0]);

    // Main content area - logs
    let log_items: Vec<ListItem> = app.logs
        .iter()
        .map(|log| ListItem::new(log.as_str()))
        .collect();
    
    // Create a stateful list that scrolls to bottom
    let mut state = ListState::default();
    if !app.logs.is_empty() {
        state.select(Some(app.logs.len().saturating_sub(1)));
    }
    
    let log_list = List::new(log_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Logs"))
        .style(Style::default().fg(Color::White));
    frame.render_stateful_widget(log_list, chunks[1], &mut state);

    // Footer with help
    let help_text = if app.building {
        "Building package... (Press 'q' to quit)"
    } else {
        "Press 'b' to build and install package | 'q' to quit"
    };
    let footer = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Yellow))
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(footer, chunks[2]);
}
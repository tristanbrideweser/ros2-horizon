// src/ui.rs

use ratatui::{
    layout::{
        Constraint, 
        Direction,
        Layout,
    }, 
    widgets::*,
    style::*, 
    Frame
};
use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    // Create main layout: left panel (30%) and right side (70%)
    let outer_layer = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(frame.area());

    let left_panel = outer_layer[0];
    let right_area = outer_layer[1];

    // Split right side into top and bottom (50/50)
    let inner_right_layer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(right_area);

    let right_top = inner_right_layer[0];
    let right_bottom = inner_right_layer[1];

    // Determine border style based on active panel
    let left_border_style = if app.active_panel == crate::app::ActivePanel::Left {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let right_top_border_style = if app.active_panel == crate::app::ActivePanel::RightTop {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let right_bottom_border_style = if app.active_panel == crate::app::ActivePanel::RightBottom {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    // Render left panel - Packages/Nodes/Topics list
    let left_items: Vec<ListItem> = app.topics
        .iter()
        .enumerate()
        .map(|(i, topic)| {
            let style = if i == app.selected_index {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(topic.as_str()).style(style)
        })
        .collect();
    
    let left_list = List::new(left_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Packages/Nodes")
            .border_style(left_border_style));
    frame.render_widget(left_list, left_panel);

    // Render right top panel - Details/Info
    let info_text = if app.building {
        "Building package...\n\nRunning: colcon build --symlink-install"
    } else {
        "LazyROS - ROS2 TUI\n\nPress 'b' to build packages\nPress 'Tab' / 'Shift+Tab' to switch panels\nPress 'q' to quit"
    };
    
    let info_paragraph = Paragraph::new(info_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Info")
            .border_style(right_top_border_style))
        .wrap(Wrap { trim: true });
    frame.render_widget(info_paragraph, right_top);

    // Render right bottom panel - Logs/Build output
    let log_items: Vec<ListItem> = app.logs
        .iter()
        .map(|log| ListItem::new(log.as_str()))
        .collect();
    
    let mut log_state = ListState::default();
    if !app.logs.is_empty() {
        log_state.select(Some(app.logs.len().saturating_sub(1)));
    }
    
    let log_list = List::new(log_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Logs")
            .border_style(right_bottom_border_style));
    frame.render_stateful_widget(log_list, right_bottom, &mut log_state);
}
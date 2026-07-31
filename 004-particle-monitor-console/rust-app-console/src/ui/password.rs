use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_password(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Old Password
            Constraint::Length(3), // New Password
            Constraint::Length(3), // Confirm Password
            Constraint::Length(3), // Set Password Button
            Constraint::Length(3), // Clear Password Button (Super Password)
            Constraint::Min(2),
        ])
        .split(area);

    let mask_str = |s: &str| "*".repeat(s.len());

    // Field 0: Old Password
    let old_style = if app.active_field == 0 {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let old_p = Paragraph::new(format!(" {}", mask_str(&app.input_old_pwd)))
        .style(old_style)
        .block(Block::default().borders(Borders::ALL).title(" Old Password "));
    f.render_widget(old_p, chunks[0]);

    // Field 1: New Password
    let new_style = if app.active_field == 1 {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let new_p = Paragraph::new(format!(" {}", mask_str(&app.input_new_pwd)))
        .style(new_style)
        .block(Block::default().borders(Borders::ALL).title(" New Password (min 3 chars) "));
    f.render_widget(new_p, chunks[1]);

    // Field 2: Confirm Password
    let conf_style = if app.active_field == 2 {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let conf_p = Paragraph::new(format!(" {}", mask_str(&app.input_confirm_pwd)))
        .style(conf_style)
        .block(Block::default().borders(Borders::ALL).title(" Confirm Password "));
    f.render_widget(conf_p, chunks[2]);

    // Action 3: Set Password
    let set_style = if app.active_field == 3 {
        Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green)
    };
    let set_p = Paragraph::new(" [ SET NEW PASSWORD ] (Press [Enter] to apply)")
        .style(set_style)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(set_p, chunks[3]);

    // Action 4: Clear Password
    let clear_style = if app.active_field == 4 {
        Style::default().fg(Color::Black).bg(Color::Red).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Red)
    };
    let clear_p = Paragraph::new(" [ CLEAR PASSWORD / DISABLE PROTECTION ] (Requires Super Password)")
        .style(clear_style)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(clear_p, chunks[4]);

    let hint_p = Paragraph::new(" Use [Tab] / [Up/Down] to navigate | Type password directly | [Enter] execute action")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(hint_p, chunks[5]);
}

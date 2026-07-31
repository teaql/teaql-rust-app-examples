use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_setting(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Auth banner
            Constraint::Length(3), // Calibration Point
            Constraint::Length(3), // Data Keep Days
            Constraint::Length(3), // Sampling Frequency
            Constraint::Length(3), // Submit hint / Action
            Constraint::Min(2),
        ])
        .split(area);

    let password_enabled = app.settings.as_ref().map_or(false, |s| s.password_enabled() == 1);

    // 1. Auth Status Banner
    let (auth_msg, auth_color) = if !password_enabled {
        ("Password Protection: DISABLED (Settings Unlocked)".to_string(), Color::Green)
    } else if app.is_authenticated {
        ("Password Protection: AUTHENTICATED (Settings Unlocked)".to_string(), Color::LightGreen)
    } else {
        ("Password Protection: LOCKED (Press [L] or [Enter] to Login)".to_string(), Color::Red)
    };

    let auth_p = Paragraph::new(auth_msg)
        .style(Style::default().fg(auth_color).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title(" Security Status "));
    f.render_widget(auth_p, chunks[0]);

    if password_enabled && !app.is_authenticated {
        let lock_notice = Paragraph::new(
            "\n  🔒 Device Settings are currently locked by system security policy.\n\n  Press [L] or [Enter] to open login dialog and enter password.",
        )
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title(" Access Restricted "));
        f.render_widget(lock_notice, chunks[1]);
        return;
    }

    // 2. Field 0: Calibration Point
    let cal_style = if app.active_field == 0 {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let cal_p = Paragraph::new(format!(" {}", app.input_calibration))
        .style(cal_style)
        .block(Block::default().borders(Borders::ALL).title(" Calibration Point (0-Point Offset) "));
    f.render_widget(cal_p, chunks[1]);

    // 3. Field 1: Data Keep Days
    let keep_style = if app.active_field == 1 {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let keep_p = Paragraph::new(format!(" {} days", app.input_keep_days))
        .style(keep_style)
        .block(Block::default().borders(Borders::ALL).title(" Data Retention Period (Days) "));
    f.render_widget(keep_p, chunks[2]);

    // 4. Field 2: Sampling Frequency
    let freq_style = if app.active_field == 2 {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let freq_p = Paragraph::new(format!(" {} seconds", app.input_sampling_freq))
        .style(freq_style)
        .block(Block::default().borders(Borders::ALL).title(" Sampling Interval / Frequency (Seconds) "));
    f.render_widget(freq_p, chunks[3]);

    // 5. Submit Button / Instructions
    let submit_style = if app.active_field == 3 {
        Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Cyan)
    };
    let submit_p = Paragraph::new(" [ SUBMIT & SAVE SETTINGS ] (Press [Enter] on focus to save)")
        .style(submit_style)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(submit_p, chunks[4]);

    let hint_p = Paragraph::new(" Use [Tab] or [Up/Down] to navigate fields | [Backspace] edit | [Enter] submit")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(hint_p, chunks[5]);
}

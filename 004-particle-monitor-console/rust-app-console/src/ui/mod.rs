pub mod calendar;
pub mod chart;
pub mod history;
pub mod home;
pub mod password;
pub mod setting;

use crate::app::{App, ModalState};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Tabs},
    Frame,
};

pub fn render_ui(f: &mut Frame, app: &App) {
    let size = f.area();

    // Top-level Vertical Layout: Header (3), Main Body (Min), Footer (1)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(1),
        ])
        .split(size);

    // 1. Header with Tabs
    let titles = vec![
        " 1: Home ",
        " 2: Chart ",
        " 3: History ",
        " 4: Setting ",
        " 5: Password ",
    ];

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" PMS Cleanroom Particle Monitor System [TeaQL + Ratatui Rust] "),
        )
        .select(app.active_tab)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(tabs, chunks[0]);

    // 2. Main Body Tabs
    match app.active_tab {
        0 => home::render_home(f, app, chunks[1]),
        1 => chart::render_chart(f, app, chunks[1]),
        2 => history::render_history(f, app, chunks[1]),
        3 => setting::render_setting(f, app, chunks[1]),
        4 => password::render_password(f, app, chunks[1]),
        _ => {}
    }

    // 3. Footer Bar
    let status_text = format!(
        " Status: {} | Selected Date: {} | Connected: SQLite Memory",
        app.status_message,
        app.calendar.selected_date.format("%Y-%m-%d")
    );
    let footer = Paragraph::new(status_text)
        .style(Style::default().fg(Color::Black).bg(Color::Cyan));
    f.render_widget(footer, chunks[2]);

    // 4. Modal Overlays
    if let Some(ref modal) = app.modal {
        render_modal(f, app, modal, size);
    }
}

fn render_modal(f: &mut Frame, app: &App, modal: &ModalState, area: Rect) {
    let popup_area = centered_rect(60, 30, area);
    f.render_widget(Clear, popup_area); // Clear background behind modal

    match modal {
        ModalState::Login => {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Length(3)])
                .split(popup_area);

            let title_p = Paragraph::new(" Security Password Verification Required")
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title(" LOGIN "));
            f.render_widget(title_p, layout[0]);

            let mask_pwd = "*".repeat(app.login_input.len());
            let input_p = Paragraph::new(format!(" Password: {}", mask_pwd))
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .block(Block::default().borders(Borders::ALL).title(" Enter Password & Press [Enter] "));
            f.render_widget(input_p, layout[1]);
        }
        ModalState::SuperPasswordClear => {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Length(3)])
                .split(popup_area);

            let title_p = Paragraph::new(" Super Password Required to Disable Protection")
                .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title(" DISABLE PASSWORD "));
            f.render_widget(title_p, layout[0]);

            let mask_pwd = "*".repeat(app.super_pwd_input.len());
            let input_p = Paragraph::new(format!(" Super Password: {}", mask_pwd))
                .style(Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD))
                .block(Block::default().borders(Borders::ALL).title(" Enter Super Password & Press [Enter] "));
            f.render_widget(input_p, layout[1]);
        }
        ModalState::Notice(msg) => {
            let text = vec![
                Line::from(vec![Span::styled(msg, Style::default().fg(Color::Green))]),
                Line::from(""),
                Line::from("Press [Enter] or [Esc] to dismiss"),
            ];
            let p = Paragraph::new(text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title(" INFO "));
            f.render_widget(p, popup_area);
        }
        ModalState::Error(msg) => {
            let text = vec![
                Line::from(vec![Span::styled(msg, Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))]),
                Line::from(""),
                Line::from("Press [Enter] or [Esc] to dismiss"),
            ];
            let p = Paragraph::new(text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title(" ERROR "));
            f.render_widget(p, popup_area);
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

use crate::app::App;
use crate::ui::calendar;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame,
};

pub fn render_history(f: &mut Frame, app: &App, area: Rect) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(34), // Left pane: Calendar Date Picker Widget
            Constraint::Min(10),    // Right pane: Data Table
        ])
        .split(area);

    // 1. Render Calendar Widget on the Left Pane
    calendar::render_calendar(f, &app.calendar, main_layout[0]);

    // 2. Right Pane: History Data Table filtered by Selected Date
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(3)])
        .split(main_layout[1]);

    let filtered_samples = app.get_filtered_samples();

    let header_cells = [
        "#", "Sample Time", "Gas", "LRef", "C1", "C2", "C3", "C4", "C5", "C6", "C7", "C8",
    ]
    .iter()
    .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = filtered_samples.iter().enumerate().map(|(idx, s)| {
        let time_str = s.sample_time().format("%H:%M:%S").to_string();

        let style = if idx == app.history_selected_index {
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let cells = vec![
            Cell::from((idx + 1).to_string()),
            Cell::from(time_str),
            Cell::from(s.gas().to_string()),
            Cell::from(s.lref().to_string()),
            Cell::from(s.impurity1().to_string()),
            Cell::from(s.impurity2().to_string()),
            Cell::from(s.impurity3().to_string()),
            Cell::from(s.impurity4().to_string()),
            Cell::from(s.impurity5().to_string()),
            Cell::from(s.impurity6().to_string()),
            Cell::from(s.impurity7().to_string()),
            Cell::from(s.impurity8().to_string()),
        ];
        Row::new(cells).style(style).height(1)
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(4),
            Constraint::Length(10),
            Constraint::Length(6),
            Constraint::Length(6),
            Constraint::Length(6),
            Constraint::Length(6),
            Constraint::Length(6),
            Constraint::Length(6),
            Constraint::Length(6),
            Constraint::Length(6),
            Constraint::Length(6),
            Constraint::Length(6),
        ],
    )
    .header(header)
    .block(
        Block::default().borders(Borders::ALL).title(format!(
            " Sampling Records for {} ({}) ",
            app.calendar.selected_date.format("%Y-%m-%d"),
            filtered_samples.len()
        )),
    );

    let mut state = TableState::default();
    if !filtered_samples.is_empty() {
        state.select(Some(app.history_selected_index.min(filtered_samples.len() - 1)));
    }

    f.render_stateful_widget(table, right_chunks[0], &mut state);

    let hint_p = Paragraph::new(" Click Calendar Date on Left | [↑/↓] Navigate rows | Mouse Scroll support")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(hint_p, right_chunks[1]);
}

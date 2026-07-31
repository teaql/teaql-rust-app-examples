use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

pub fn render_home(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Metrics header cards
            Constraint::Min(8),    // Channel data table
            Constraint::Length(3), // Key hints footer
        ])
        .split(area);

    // 1. Metrics Header Cards
    let latest_sample = app.samples.first();
    let (sample_time, gas_str, lref_str) = if let Some(s) = latest_sample {
        (s.sample_time().format("%Y-%m-%d %H:%M:%S").to_string(), s.gas().to_string(), s.lref().to_string())
    } else {
        ("N/A".to_string(), "0.0".to_string(), "0.0".to_string())
    };

    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
        .split(chunks[0]);

    let time_p = Paragraph::new(format!(" Timestamp: {}", sample_time))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title(" Sample Time "));
    f.render_widget(time_p, header_chunks[0]);

    let gas_p = Paragraph::new(format!(" {} L/min", gas_str))
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title(" Gas Flow Rate "));
    f.render_widget(gas_p, header_chunks[1]);

    let lref_p = Paragraph::new(format!(" {} V", lref_str))
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title(" LRef Voltage "));
    f.render_widget(lref_p, header_chunks[2]);

    // 2. Channel Data Table
    let (c1, c2, c3, c4, c5, c6, c7, c8) = if let Some(s) = latest_sample {
        (
            s.impurity1(),
            s.impurity2(),
            s.impurity3(),
            s.impurity4(),
            s.impurity5(),
            s.impurity6(),
            s.impurity7(),
            s.impurity8(),
        )
    } else {
        (0, 0, 0, 0, 0, 0, 0, 0)
    };

    // Calculate Differential and Cumulative counts for channels
    let channel_data = [
        ("0.100 um", c1 - c2, c1),
        ("0.200 um", c2 - c3, c2),
        ("0.300 um", c3 - c4, c3),
        ("0.500 um", c4 - c5, c4),
        ("1.000 um", c5 - c6, c5),
        ("2.000 um", c6 - c7, c6),
        ("3.000 um", c7 - c8, c7),
        ("5.000 um", c8, c8),
    ];

    let header_cells = ["Channel Size (μm)", "Differential Count", "Cumulative Count"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = channel_data.iter().map(|(size, diff, cum)| {
        let cells = vec![
            Cell::from(*size).style(Style::default().fg(Color::White)),
            Cell::from(diff.to_string()).style(Style::default().fg(Color::LightCyan)),
            Cell::from(cum.to_string()).style(Style::default().fg(Color::LightGreen)),
        ];
        Row::new(cells).height(1)
    });

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(30),
            Constraint::Percentage(35),
            Constraint::Percentage(35),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Real-time Particle Channel Spectrum "),
    );

    f.render_widget(table, chunks[1]);

    // 3. Footer Control Hint
    let hint_p = Paragraph::new(" Press [S] or [Space] to trigger real-time sampling | [1-5] Switch Tabs | [Q] Quit")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(hint_p, chunks[2]);
}

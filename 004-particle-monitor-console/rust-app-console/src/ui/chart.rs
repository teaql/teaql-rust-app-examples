use crate::app::App;
use crate::ui::calendar;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph},
    Frame,
};

pub fn render_chart(f: &mut Frame, app: &App, area: Rect) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(34), // Left pane: Calendar Date Picker Widget
            Constraint::Min(10),    // Right pane: Chart
        ])
        .split(area);

    // 1. Render Calendar Widget on Left Pane
    calendar::render_calendar(f, &app.calendar, main_layout[0]);

    // 2. Right Pane: Chart
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(3)])
        .split(main_layout[1]);

    let filtered_samples = app.get_filtered_samples();

    if filtered_samples.is_empty() {
        let empty_p = Paragraph::new(format!(
            "\n  No historical sample data recorded on {}",
            app.calendar.selected_date.format("%Y-%m-%d")
        ))
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Particle Trend Chart "),
        );
        f.render_widget(empty_p, right_chunks[0]);
        return;
    }

    // Convert filtered samples to chart coordinates (oldest -> newest)
    let reversed_samples: Vec<_> = filtered_samples.iter().copied().rev().collect();

    let data_c1: Vec<(f64, f64)> = reversed_samples
        .iter()
        .enumerate()
        .map(|(idx, s)| (idx as f64, s.impurity1() as f64))
        .collect();

    let data_c4: Vec<(f64, f64)> = reversed_samples
        .iter()
        .enumerate()
        .map(|(idx, s)| (idx as f64, s.impurity4() as f64))
        .collect();

    let data_c5: Vec<(f64, f64)> = reversed_samples
        .iter()
        .enumerate()
        .map(|(idx, s)| (idx as f64, s.impurity5() as f64))
        .collect();

    let max_x = (reversed_samples.len() as f64 - 1.0).max(10.0);
    let max_y_c1 = data_c1.iter().map(|(_, y)| *y).fold(100.0, f64::max);

    let datasets = vec![
        Dataset::default()
            .name("0.1μm (C1)")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Cyan))
            .data(&data_c1),
        Dataset::default()
            .name("0.5μm (C4)")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Yellow))
            .data(&data_c4),
        Dataset::default()
            .name("1.0μm (C5)")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Magenta))
            .data(&data_c5),
    ];

    let x_labels = vec![
        "Start".to_string(),
        format!("Samples ({})", reversed_samples.len()),
        "Latest".to_string(),
    ];

    let y_max_str = format!("{:.0}", max_y_c1);
    let y_labels = vec!["0".to_string(), format!("{:.0}", max_y_c1 / 2.0), y_max_str];

    let chart = Chart::new(datasets)
        .block(Block::default().borders(Borders::ALL).title(format!(
            " Particle Concentration Trend for {} ",
            app.calendar.selected_date.format("%Y-%m-%d")
        )))
        .x_axis(
            Axis::default()
                .title("Timeline / Sample Sequence")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, max_x])
                .labels(x_labels),
        )
        .y_axis(
            Axis::default()
                .title("Counts / L")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, max_y_c1 * 1.1])
                .labels(y_labels),
        );

    f.render_widget(chart, right_chunks[0]);

    let hint_p = Paragraph::new(" Click Calendar Date on Left | Cyan: 0.1μm | Yellow: 0.5μm | Magenta: 1.0μm")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(hint_p, right_chunks[1]);
}

use chrono::{Datelike, Local, NaiveDate};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct CalendarState {
    pub selected_date: NaiveDate,
    pub view_year: i32,
    pub view_month: u32,
}

impl CalendarState {
    pub fn new() -> Self {
        let today = Local::now().date_naive();
        Self {
            selected_date: today,
            view_year: today.year(),
            view_month: today.month(),
        }
    }

    pub fn prev_month(&mut self) {
        if self.view_month == 1 {
            self.view_month = 12;
            self.view_year -= 1;
        } else {
            self.view_month -= 1;
        }
    }

    pub fn next_month(&mut self) {
        if self.view_month == 12 {
            self.view_month = 1;
            self.view_year += 1;
        } else {
            self.view_month += 1;
        }
    }

    pub fn select_day(&mut self, day: u32) {
        if let Some(date) = NaiveDate::from_ymd_opt(self.view_year, self.view_month, day) {
            self.selected_date = date;
        }
    }

    pub fn handle_mouse_click(&mut self, col: u16, row: u16, area: Rect) -> bool {
        if !area.contains(ratatui::layout::Position::new(col, row)) {
            return false;
        }

        let rel_x = col.saturating_sub(area.x);
        let rel_y = row.saturating_sub(area.y);

        // Header controls (Prev month <, Next month >) on line 1
        if rel_y == 1 {
            if rel_x <= 4 {
                self.prev_month();
                return true;
            } else if rel_x >= area.width.saturating_sub(5) {
                self.next_month();
                return true;
            }
        }

        // Days Grid: Row 3 is Weekday names (Mo Tu We Th Fr Sa Su)
        // Rows 4..10 are Day cells (each cell is 4 chars wide: " 01 ")
        if rel_y >= 4 && rel_y <= 9 {
            let grid_row = (rel_y - 4) as u32;
            let grid_col = ((rel_x.saturating_sub(2)) / 4) as u32;

            if grid_col < 7 {
                let first_day = NaiveDate::from_ymd_opt(self.view_year, self.view_month, 1);
                if let Some(first) = first_day {
                    let first_weekday = first.weekday().num_days_from_monday(); // 0 = Mon
                    let cell_idx = grid_row * 7 + grid_col;
                    if cell_idx >= first_weekday {
                        let day = cell_idx - first_weekday + 1;
                        let days_in_month = days_in_month(self.view_year, self.view_month);
                        if day <= days_in_month {
                            self.select_day(day);
                            return true;
                        }
                    }
                }
            }
        }

        false
    }
}

pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

pub fn render_calendar(f: &mut Frame, state: &CalendarState, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" 📅 Date Picker ");
    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Month Header [< July 2026 >]
            Constraint::Length(1), // Weekday Names [Mo Tu We Th Fr Sa Su]
            Constraint::Min(6),    // Days Grid
            Constraint::Length(1), // Selected Date Info
        ])
        .split(inner);

    // 1. Month Header
    let month_name = match state.view_month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "",
    };

    let header_line = Line::from(vec![
        Span::styled(" [<] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(
            format!(" {:^16} ", format!("{} {}", month_name, state.view_year)),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" [>] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
    ]);
    f.render_widget(Paragraph::new(header_line), chunks[0]);

    // 2. Weekday Header
    let weekdays_line = Line::from(vec![
        Span::styled("  Mo  Tu  We  Th  Fr ", Style::default().fg(Color::White)),
        Span::styled(" Sa  Su ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
    ]);
    f.render_widget(Paragraph::new(weekdays_line), chunks[1]);

    // 3. Days Grid
    let first_day = NaiveDate::from_ymd_opt(state.view_year, state.view_month, 1);
    let first_weekday = first_day.map_or(0, |d| d.weekday().num_days_from_monday());
    let total_days = days_in_month(state.view_year, state.view_month);

    let mut lines = Vec::new();
    let mut day_counter = 1;

    for week in 0..6 {
        let mut spans = vec![Span::raw(" ")];
        for day_of_week in 0..7 {
            let cell_idx = week * 7 + day_of_week;
            if cell_idx < first_weekday || day_counter > total_days {
                spans.push(Span::raw("    "));
            } else {
                let is_selected = state.selected_date.year() == state.view_year
                    && state.selected_date.month() == state.view_month
                    && state.selected_date.day() == day_counter;

                let is_weekend = day_of_week >= 5;

                let style = if is_selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else if is_weekend {
                    Style::default().fg(Color::LightYellow)
                } else {
                    Style::default().fg(Color::LightCyan)
                };

                spans.push(Span::styled(format!(" {:>2} ", day_counter), style));
                day_counter += 1;
            }
        }
        lines.push(Line::from(spans));
        if day_counter > total_days && week >= 3 {
            break;
        }
    }

    f.render_widget(Paragraph::new(lines), chunks[2]);

    // 4. Selected Date Display
    let selected_str = format!(" Selected: {}", state.selected_date.format("%Y-%m-%d"));
    let footer_p = Paragraph::new(selected_str)
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));
    f.render_widget(footer_p, chunks[3]);
}

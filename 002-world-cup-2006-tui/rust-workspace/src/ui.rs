use crate::app::{App, KnockoutMatchView, View};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};
use unicode_width::UnicodeWidthStr;

/// Pad a string to `target_width` display columns using spaces.
/// Uses the same `unicode-width` crate that ratatui uses internally,
/// ensuring our padding matches ratatui's span positioning exactly.
fn pad_right(s: &str, target_width: usize) -> String {
    let dw = UnicodeWidthStr::width(s);
    if dw >= target_width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(target_width - dw))
    }
}

fn get_block(title: String, is_active: bool) -> Block<'static> {
    let mut b = Block::default().borders(Borders::ALL).title(title);
    if is_active {
        b = b.border_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );
    }
    b
}

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Main content
            Constraint::Length(3), // Input area
        ])
        .split(f.area());

    // Header
    let header_text = match &app.view {
        View::Global => "World Cup 2026 - Knockout Bracket Dashboard".to_string(),
        View::Players => "World Cup 2026 - Terminal Dashboard (Players)".to_string(),
        View::Logs => "World Cup 2026 - System Logs".to_string(),
    };
    let header = Paragraph::new(header_text)
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Main content
    match app.view.clone() {
        View::Global => render_global(f, app, chunks[1]),
        View::Players => render_players(f, app, chunks[1]),
        View::Logs => render_logs(f, app, chunks[1]),
    }

    // Input area
    let input = Paragraph::new(format!("> {}", app.input_buffer))
        .style(Style::default().fg(Color::Cyan))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Commands: bracket | players | sync live | logs | quit"),
        );
    f.render_widget(input, chunks[2]);

    #[allow(deprecated)]
    f.set_cursor_position((
        chunks[2].x + 3 + app.input_buffer.chars().count() as u16,
        chunks[2].y + 1,
    ));
}

/// Extract a short team name from "🇨🇦 Canada" → "CAN"
pub fn team_short(name: &str) -> String {
    // Skip emoji flag (first token), use remaining name
    let parts: Vec<&str> = name.split_whitespace().collect();
    let team_name = if parts.len() > 1 {
        parts[1..].join(" ")
    } else {
        name.to_string()
    };
    // Only abbreviate names longer than 12 chars (bracket column width)
    match team_name.as_str() {
        "United States" => "USA".to_string(),
        "Bosnia and Herzegovina" => "BIH".to_string(),
        "Czech Republic" => "CZE".to_string(),
        other => {
            let s = other.split_whitespace().last().unwrap_or("?").to_string();
            if s.len() > 12 { s[..12].to_string() } else { s }
        }
    }
}

/// Extract the emoji flag from a team name like "🇨🇦 Canada" → "🇨🇦"
pub fn team_flag(name: &str) -> String {
    let parts: Vec<&str> = name.split_whitespace().collect();
    if parts.len() > 1 {
        parts[0].to_string()
    } else {
        // No flag found, fall back to short name
        team_short(name)
    }
}

fn render_bracket_tree(f: &mut Frame, matches: &[KnockoutMatchView], area: Rect, active: bool) {
    use std::collections::BTreeMap;

    if matches.is_empty() {
        let msg = Paragraph::new("No knockout matches yet. Use 'sync live' to load data.")
            .style(Style::default().fg(Color::DarkGray))
            .block(get_block("Knockout Bracket".to_string(), active));
        f.render_widget(msg, area);
        return;
    }

    let mut stages: BTreeMap<usize, Vec<&KnockoutMatchView>> = BTreeMap::new();
    for m in matches {
        stages.entry(m.stage_rank).or_default().push(m);
    }

    // Data is already reordered by fetch_knockout_matches to match bracket structure
    let r32 = stages.get(&1).cloned().unwrap_or_default();
    let r16 = stages.get(&2).cloned().unwrap_or_default();
    let qf = stages.get(&3).cloned().unwrap_or_default();

    let r32_count = r32.len();
    if r32_count == 0 {
        let msg = Paragraph::new("No R32 matches found.")
            .style(Style::default().fg(Color::DarkGray))
            .block(get_block("Knockout Bracket".to_string(), active));
        f.render_widget(msg, area);
        return;
    }

    // 16 matches, 1 line each, with 1 gap row between entries → 2*16-1 = 31 rows
    let nrows = 2 * r32_count - 1;

    // Col 0 + Col 1: match i at row 2*i
    let col1_pos: Vec<usize> = (0..r32_count).map(|i| 2 * i).collect();

    // Col 2 (8强): centered between pairs from col 1
    let col2_pos: Vec<usize> = (0..r32_count / 2)
        .map(|j| (col1_pos[2 * j] + col1_pos[2 * j + 1]) / 2)
        .collect();

    // Col 3 (4强): centered between pairs from col 2
    let col3_pos: Vec<usize> = (0..col2_pos.len() / 2)
        .map(|k| (col2_pos[2 * k] + col2_pos[2 * k + 1]) / 2)
        .collect();

    // Winner name lists
    let winner_short = |m: &KnockoutMatchView| -> String {
        m.winner
            .as_ref()
            .map(|w| team_short(w))
            .unwrap_or_else(|| "---".to_string())
    };
    let col1_names: Vec<String> = r32.iter().map(|m| winner_short(m)).collect();
    let col2_names: Vec<String> = r16.iter().map(|m| winner_short(m)).collect();
    let col3_names: Vec<String> = qf.iter().map(|m| winner_short(m)).collect();

    // Connectors
    #[derive(Clone, Copy, PartialEq)]
    enum Conn {
        Empty,
        Top,  // ─┐
        Bot,  // ─┘
        Mid,  // ├─
        Vert, //  │
    }

    let build_conn = |src: &[usize], dst: &[usize], n: usize| -> Vec<Conn> {
        let mut c = vec![Conn::Empty; n];
        for j in 0..dst.len() {
            let t = src[2 * j];
            let b = src[2 * j + 1];
            let m = dst[j];
            if t < n { c[t] = Conn::Top; }
            if b < n { c[b] = Conn::Bot; }
            if m < n { c[m] = Conn::Mid; }
            for r in (t + 1)..b {
                if r < n && c[r] == Conn::Empty { c[r] = Conn::Vert; }
            }
        }
        c
    };

    let conn12 = build_conn(&col1_pos, &col2_pos, nrows);
    let conn23 = build_conn(&col2_pos, &col3_pos, nrows);

    // Column widths
    let avail = area.width.saturating_sub(2) as usize;
    let cw = 2_usize; // connector width
    // col0 (match) + gap + col1 + conn + col2 + conn + col3
    let col0_w = ((avail.saturating_sub(cw * 2 + 2)) * 38 / 100).max(16).min(24);
    let rest = avail.saturating_sub(col0_w + cw * 2 + 2);
    let sw = (rest / 3).max(4).min(12);

    // Format match line using flag emojis: "🇨🇦 1-0 🇿🇦"
    let fmt_match = |m: &KnockoutMatchView, w: usize| -> (String, Style) {
        let h = team_flag(&m.home);
        let a = team_flag(&m.away);
        let text = format!("{} {} {}", h, m.score, a);
        let style = if m.completed {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::White)
        };
        (pad_right(&text, w), style)
    };

    let mut lines: Vec<Line> = Vec::new();

    // Header
    lines.push(Line::from(vec![
        Span::styled(pad_right("R32", col0_w), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("  "),
        Span::styled(pad_right("R16", sw), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("  "),
        Span::styled(pad_right("QF", sw), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("  "),
        Span::styled(pad_right("SF", sw), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
    ]));
    lines.push(Line::from(Span::styled(
        "─".repeat((col0_w + sw * 3 + cw * 2 + 2).min(avail)),
        Style::default().fg(Color::DarkGray),
    )));

    // Data rows
    for r in 0..nrows {
        let mut spans: Vec<Span> = Vec::new();
        let is_match_row = r % 2 == 0;

        // Col 0: R32 match
        // Emoji flags render slightly wider in terminals than unicode-width reports.
        // Match rows have 2 flags → blank rows need +1 extra padding to keep connectors aligned.
        const FLAG_OFFSET: usize = 1;
        let mi = r / 2;
        if r % 2 == 0 && mi < r32.len() {
            let (text, style) = fmt_match(r32[mi], col0_w);
            spans.push(Span::styled(text, style));
        } else {
            spans.push(Span::raw(" ".repeat(col0_w + FLAG_OFFSET)));
        }

        spans.push(Span::raw("  "));

        // Col 1: R16
        if let Some(idx) = col1_pos.iter().position(|&p| p == r) {
            if idx < col1_names.len() {
                let n = &col1_names[idx];
                let st = if n == "---" { Style::default().fg(Color::DarkGray) } else { Style::default().fg(Color::Green) };
                spans.push(Span::styled(format!("{:<w$}", n, w = sw), st));
            } else {
                spans.push(Span::raw(" ".repeat(sw)));
            }
        } else {
            spans.push(Span::raw(" ".repeat(sw)));
        }

        // Connector 1→2
        let (cs, cst) = match conn12[r] {
            Conn::Top   => ("─┐", Style::default().fg(Color::DarkGray)),
            Conn::Bot   => ("─┘", Style::default().fg(Color::DarkGray)),
            Conn::Mid   => ("├─", Style::default().fg(Color::DarkGray)),
            Conn::Vert if is_match_row => ("  │", Style::default().fg(Color::DarkGray)),
            Conn::Vert  => (" │", Style::default().fg(Color::DarkGray)),
            Conn::Empty if is_match_row => ("   ", Style::default()),
            Conn::Empty => ("  ", Style::default()),
        };
        spans.push(Span::styled(cs, cst));

        // Col 2: QF
        if let Some(idx) = col2_pos.iter().position(|&p| p == r) {
            if idx < col2_names.len() {
                let n = &col2_names[idx];
                let st = if n == "---" { Style::default().fg(Color::DarkGray) } else { Style::default().fg(Color::Cyan) };
                spans.push(Span::styled(format!("{:<w$}", n, w = sw), st));
            } else {
                spans.push(Span::raw(" ".repeat(sw)));
            }
        } else {
            spans.push(Span::raw(" ".repeat(sw)));
        }

        // Connector 2→3
        // On match rows (even), col0 has emoji flags that render wider, but FLAG_OFFSET
        // only pads blank rows. So conn23 Vert (│) on match rows needs an extra space
        // to align with conn23 Top/Mid/Bot on blank rows.
        let (cs2, cst2) = match conn23[r] {
            Conn::Top   => ("─┐", Style::default().fg(Color::DarkGray)),
            Conn::Bot   => ("─┘", Style::default().fg(Color::DarkGray)),
            Conn::Mid   => (" ├─", Style::default().fg(Color::DarkGray)),
            Conn::Vert if is_match_row => ("  │", Style::default().fg(Color::DarkGray)),
            Conn::Vert  => (" │", Style::default().fg(Color::DarkGray)),
            Conn::Empty if is_match_row => ("   ", Style::default()),
            Conn::Empty => ("  ", Style::default()),
        };
        spans.push(Span::styled(cs2, cst2));

        // Col 3: SF
        if let Some(idx) = col3_pos.iter().position(|&p| p == r) {
            if idx < col3_names.len() {
                let n = &col3_names[idx];
                let st = if n == "---" { Style::default().fg(Color::DarkGray) } else { Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD) };
                spans.push(Span::styled(format!("{:<w$}", n, w = sw), st));
            } else {
                spans.push(Span::raw(" ".repeat(sw)));
            }
        } else {
            spans.push(Span::raw(" ".repeat(sw)));
        }

        lines.push(Line::from(spans));
    }

    // Final + 3rd Place
    let final_m = stages.get(&6).and_then(|v| v.first());
    let third_m = stages.get(&5).and_then(|v| v.first());
    lines.push(Line::from(""));

    if let Some(fm) = final_m {
        let champ = fm.winner.as_deref().unwrap_or("TBD");
        lines.push(Line::from(vec![
            Span::styled("🏆 Final: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{} {} {}", team_short(&fm.home), fm.score, team_short(&fm.away)),
                if fm.completed { Style::default().fg(Color::Green) } else { Style::default().fg(Color::White) },
            ),
            Span::styled(format!("  Champion: {}", champ), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]));
    }
    if let Some(tm) = third_m {
        lines.push(Line::from(vec![
            Span::styled("🥉 3rd:   ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{} {} {}", team_short(&tm.home), tm.score, team_short(&tm.away)),
                if tm.completed { Style::default().fg(Color::Green) } else { Style::default().fg(Color::White) },
            ),
        ]));
    }

    let done = matches.iter().filter(|m| m.completed).count();
    let title = format!("Knockout Bracket  {}/{} completed", done, matches.len());
    let tree = Paragraph::new(lines).block(get_block(title, active));
    f.render_widget(tree, area);
}

fn render_global(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(area);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Left: Vertical bracket tree
    render_bracket_tree(f, &app.knockout_matches, chunks[0], app.active_pane == 0);

    // Top Players
    let p_header = Row::new(vec!["#", "Team", "Player", "Goals"])
        .style(Style::default().add_modifier(Modifier::BOLD))
        .bottom_margin(1);
    let p_rows: Vec<Row> = app
        .top_players
        .iter()
        .enumerate()
        .map(|(i, (t, p, c))| {
            Row::new(vec![
                Cell::from((i + 1).to_string()),
                Cell::from(t.clone()),
                Cell::from(p.clone()),
                Cell::from(c.to_string()).style(Style::default().fg(Color::Yellow)),
            ])
        })
        .collect();

    let p_len = app.top_players.len();
    let p_sel = app.players_state.selected().unwrap_or(0);
    let p_title = format!(
        "Top Players  {} / {}  ↑/↓ to scroll",
        if p_len > 0 { p_sel + 1 } else { 0 },
        p_len
    );

    let p_table = Table::new(
        p_rows,
        [
            Constraint::Length(4),
            Constraint::Percentage(30),
            Constraint::Percentage(50),
            Constraint::Percentage(15),
        ],
    )
    .header(p_header)
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .block(get_block(p_title, app.active_pane == 1));
    f.render_stateful_widget(p_table, right_chunks[0], &mut app.players_state);

    // Completed Knockout Matches
    let m_header = Row::new(vec![
        Cell::from(Line::from("Home").alignment(Alignment::Right)),
        Cell::from(Line::from("Score").alignment(Alignment::Center)),
        Cell::from(Line::from("Away").alignment(Alignment::Left)),
    ])
    .style(Style::default().add_modifier(Modifier::BOLD))
    .bottom_margin(1);
    let m_rows: Vec<Row> = app
        .recent_matches
        .iter()
        .map(|m| {
            let h = m
                .home_team()
                .map(|t| format!("{} {}", t.team_name(), t.emoji_flag()))
                .unwrap_or_default();
            let a = m
                .away_team()
                .map(|t| format!("{} {}", t.emoji_flag(), t.team_name()))
                .unwrap_or_default();
            let s = format!("{} - {}", m.home_score(), m.away_score());
            Row::new(vec![
                Cell::from(Line::from(h).alignment(Alignment::Right)),
                Cell::from(Line::from(s).alignment(Alignment::Center)),
                Cell::from(Line::from(a).alignment(Alignment::Left)),
            ])
        })
        .collect();
    let m_len = app.recent_matches.len();
    let m_sel = app.matches_state.selected().unwrap_or(0);
    let m_title = format!(
        "Completed Knockout Matches  {} / {}  ↑/↓ to scroll",
        if m_len > 0 { m_sel + 1 } else { 0 },
        m_len
    );
    let m_table = Table::new(
        m_rows,
        [
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(40),
        ],
    )
    .header(m_header)
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .block(get_block(m_title, app.active_pane == 2));
    f.render_stateful_widget(m_table, right_chunks[1], &mut app.matches_state);
}

fn render_players(f: &mut Frame, app: &mut App, area: Rect) {
    let header = Row::new(vec![
        Cell::from(Line::from("Team").alignment(Alignment::Left)),
        Cell::from(Line::from("Player").alignment(Alignment::Left)),
        Cell::from(Line::from("Goals").alignment(Alignment::Center)),
        Cell::from(Line::from("Matches").alignment(Alignment::Left)),
    ])
    .style(Style::default().add_modifier(Modifier::BOLD))
    .bottom_margin(1);

    let rows: Vec<Row> = app
        .all_players
        .iter()
        .map(|(t, p, c, m)| {
            let matches_lines: Vec<Line> = m.iter().map(|ms| Line::from(ms.clone())).collect();
            let matches_text = ratatui::text::Text::from(matches_lines);
            let height = m.len().max(1) as u16;

            Row::new(vec![
                Cell::from(t.clone()),
                Cell::from(p.clone()),
                Cell::from(Line::from(c.to_string()).alignment(Alignment::Center))
                    .style(Style::default().fg(Color::Yellow)),
                Cell::from(matches_text),
            ])
            .height(height)
        })
        .collect();

    let total = app.all_players.len();
    let sel = app.player_table_state.selected().unwrap_or(0);
    let title = format!(
        "Players  {} / {}  ↑/↓ to scroll",
        if total > 0 { sel + 1 } else { 0 },
        total
    );

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Length(7),
            Constraint::Percentage(50),
        ],
    )
    .header(header)
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .block(get_block(title, true));
    f.render_stateful_widget(table, area, &mut app.player_table_state);
}

fn render_logs(f: &mut Frame, app: &mut App, area: Rect) {
    use ratatui::widgets::{List, ListItem};
    let logs = if let Ok(l) = app.logs.lock() {
        l.iter().cloned().collect::<Vec<_>>()
    } else {
        vec![]
    };

    let items: Vec<ListItem> = logs
        .iter()
        .map(|log| ListItem::new(parse_log_line(log)))
        .collect();

    let total = items.len();
    let sel = app.logs_state.selected().unwrap_or(0);
    let title = format!(
        "System Logs  {} / {}  ↑/↓ to scroll",
        if total > 0 { sel + 1 } else { 0 },
        total
    );

    let list = List::new(items)
        .block(get_block(title, true))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    f.render_stateful_widget(list, area, &mut app.logs_state);
}

pub fn parse_log_line(line: &str) -> Line<'_> {
    let mut spans = Vec::new();
    let mut rest = line;

    // Detect if this is an AUDIT line (set during level bracket parsing)
    let mut is_audit = false;

    // Detect aligned format: e.g. [08:32:31.456]-[user]-[DEBUG/AUDIT]-message
    if line.starts_with('[') && line.len() > 15 {
        if let Some(time_end) = line.find(']') {
            let timestamp = &line[1..time_end];
            // Only match if the bracket contents look like a time (contains colon)
            if timestamp.contains(':') {
                spans.push(Span::styled(
                    format!("[{}]", timestamp),
                    Style::default().fg(Color::Indexed(244)),
                ));
                rest = &line[time_end + 1..];

                // 2. User ID bracket e.g. -[user]
                if rest.starts_with("-[") {
                    if let Some(end) = rest[2..].find(']') {
                        let user_part = &rest[2..end + 2];
                        spans.push(Span::styled("-", Style::default().fg(Color::Indexed(240))));
                        spans.push(Span::styled(
                            format!("[{}]", user_part),
                            Style::default()
                                .fg(Color::Rgb(155, 89, 182))
                                .add_modifier(Modifier::BOLD),
                        ));
                        rest = &rest[end + 3..];
                    }
                }

                // 3. Severity Level bracket e.g. -[AUDIT], -[INFO], or -[DEBUG]
                if rest.starts_with("-[") {
                    if let Some(end) = rest[2..].find(']') {
                        let level = &rest[2..end + 2];
                        spans.push(Span::styled("-", Style::default().fg(Color::Indexed(240))));
                        if level == "AUDIT" {
                            is_audit = true;
                            spans.push(Span::styled(
                                format!("[{}]", level),
                                Style::default()
                                    .fg(Color::Yellow)
                                    .add_modifier(Modifier::BOLD),
                            ));
                        } else if level == "INFO" {
                            spans.push(Span::styled(
                                format!("[{}]", level),
                                Style::default()
                                    .fg(Color::Rgb(46, 204, 113))
                                    .add_modifier(Modifier::BOLD),
                            ));
                        } else {
                            spans.push(Span::styled(
                                format!("[{}]", level),
                                Style::default().fg(Color::Indexed(242)),
                            ));
                        }
                        rest = &rest[end + 3..];
                    }
                }

                if rest.starts_with('-') {
                    spans.push(Span::styled("-", Style::default().fg(Color::Indexed(240))));
                    rest = &rest[1..];
                }
            } else {
                // Fallback for indented lines or others
                spans.push(Span::styled(line, Style::default().fg(Color::White)));
                return Line::from(spans);
            }
        }
    } else {
        // Fallback for other lines
        spans.push(Span::styled(line, Style::default().fg(Color::White)));
        return Line::from(spans);
    }

    // Now highlight the rest of the message!
    // Handle extra bracket after timing (e.g. [1234µs] or [DEBUG] in reformatted SQL logs)
    if rest.starts_with("[") {
        if let Some(end) = rest[1..].find(']') {
            let tag = &rest[1..end + 1];
            // Highlight µs timing in red, other tags in gray
            let tag_style = if tag.ends_with("µs") {
                Style::default()
                    .fg(Color::Rgb(231, 76, 60))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Indexed(242))
            };
            spans.push(Span::styled(format!("[{}]", tag), tag_style));
            rest = &rest[end + 2..];
            if rest.starts_with('-') {
                spans.push(Span::styled("-", Style::default().fg(Color::Indexed(240))));
                rest = &rest[1..];
            }
        }
    }

    // Parse the next bracket too (e.g. [DEBUG] after [µs])
    if rest.starts_with("[") {
        if let Some(end) = rest[1..].find(']') {
            let tag = &rest[1..end + 1];
            let tag_style = if tag.ends_with("µs") {
                Style::default()
                    .fg(Color::Rgb(231, 76, 60))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Indexed(242))
            };
            spans.push(Span::styled(format!("[{}]", tag), tag_style));
            rest = &rest[end + 2..];
            if rest.starts_with('-') {
                spans.push(Span::styled("-", Style::default().fg(Color::Indexed(240))));
                rest = &rest[1..];
            }
        }
    }

    if rest.starts_with("SqlLogEntry") {
        spans.push(Span::styled(
            "SqlLogEntry",
            Style::default().fg(Color::Indexed(242)),
        ));
        rest = &rest[11..];
    }

    // Use AUDIT orange for the entire message body if this is an AUDIT line
    if is_audit {
        colorize_comment_segment(rest, &mut spans, Style::default().fg(Color::Yellow));
        return Line::from(spans);
    }

    // 4. Comment part and Result summary part
    if rest.starts_with(" - [") {
        if let Some(end) = rest[4..].find(']') {
            let first_segment = &rest[..end + 5];
            let after_first = &rest[end + 5..];

            if after_first.starts_with(" - [") {
                // If there is another " - [" immediately following, then the first one is the comment!
                colorize_comment_segment(
                    first_segment,
                    &mut spans,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                );
                rest = after_first;

                // Now parse the second one as the result summary
                if let Some(end2) = rest[4..].find(']') {
                    let result_part = &rest[..end2 + 5];
                    colorize_comment_segment(
                        result_part,
                        &mut spans,
                        Style::default().fg(Color::Rgb(52, 152, 219)),
                    );
                    rest = &rest[end2 + 5..];
                }
            } else {
                // If there is no " - [" following, then this first segment is the result summary (no comment exists)!
                colorize_comment_segment(
                    first_segment,
                    &mut spans,
                    Style::default().fg(Color::Rgb(52, 152, 219)),
                );
                rest = after_first;
            }
        }
    }

    // 5. Highlight Changes in Audit logs or remaining SQL
    if let Some(changes_idx) = rest.find(" Changes: ") {
        let main_msg = &rest[..changes_idx];
        let changes = &rest[changes_idx..];
        spans.push(Span::styled(main_msg, Style::default().fg(Color::White)));
        spans.push(Span::styled(changes, Style::default().fg(Color::Cyan)));
    } else if rest.starts_with("Execute TeaQL - ") {
        spans.push(Span::styled(
            "Execute TeaQL - ",
            Style::default().fg(Color::Indexed(242)),
        ));
        spans.push(Span::styled(
            rest[16..].to_owned(),
            Style::default()
                .fg(Color::Rgb(46, 204, 113))
                .add_modifier(Modifier::BOLD),
        ));
    } else if rest.starts_with("Starting business action: ") {
        spans.push(Span::styled(
            "DOMAIN: ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(
            rest[26..].to_owned(),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
    } else if rest.starts_with("Finished business action: ") {
        spans.push(Span::styled(
            "✔ ",
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(
            rest[26..].to_owned(),
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ));
    } else if rest.starts_with("Starting query: ") {
        spans.push(Span::styled(
            "▶ ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(
            rest[16..].to_owned(),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));
    } else if rest.starts_with("Finished query: ") {
        spans.push(Span::styled(
            "✔ ",
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(
            rest[16..].to_owned(),
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        ));
    } else if rest.starts_with("Business Log: ") {
        spans.push(Span::styled(
            "🛈 ",
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(
            rest[14..].to_owned(),
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        ));
    } else {
        colorize_sql(rest, &mut spans);
    }

    Line::from(spans)
}

fn colorize_comment_segment<'a>(text: &'a str, spans: &mut Vec<Span<'a>>, base_style: Style) {
    let mut current_idx = 0;
    while let Some(start) = text[current_idx..].find('(') {
        let abs_start = current_idx + start;
        if let Some(end) = text[abs_start..].find(')') {
            let abs_end = abs_start + end;
            let inner = &text[abs_start + 1..abs_end];
            if !inner.is_empty()
                && (inner.chars().all(|c| c.is_ascii_digit()) || inner == "pending")
            {
                if abs_start > current_idx {
                    spans.push(Span::styled(&text[current_idx..abs_start + 1], base_style));
                }
                spans.push(Span::styled(
                    inner,
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ));
                current_idx = abs_end;
                continue;
            }
        }
        // If not matching, just skip the '('
        spans.push(Span::styled(&text[current_idx..abs_start + 1], base_style));
        current_idx = abs_start + 1;
    }
    if current_idx < text.len() {
        spans.push(Span::styled(&text[current_idx..], base_style));
    }
}

fn colorize_sql<'a>(sql: &'a str, spans: &mut Vec<Span<'a>>) {
    let mut current_idx = 0;
    let mut text_start = 0;

    while let Some(quote_idx) = sql[current_idx..].find('\'') {
        let abs_quote = current_idx + quote_idx;
        if abs_quote > text_start {
            colorize_sql_text(&sql[text_start..abs_quote], spans);
        }

        let mut end_idx = abs_quote + 1;
        loop {
            if let Some(next_quote) = sql[end_idx..].find('\'') {
                end_idx += next_quote; // this is the index of the next quote
                if end_idx + 1 < sql.len() && sql[end_idx + 1..].starts_with('\'') {
                    end_idx += 2; // skip escaped quote
                } else {
                    end_idx += 1; // include the closing quote
                    break;
                }
            } else {
                end_idx = sql.len();
                break;
            }
        }
        spans.push(Span::styled(
            sql[abs_quote..end_idx].to_owned(),
            Style::default().fg(Color::Red),
        ));
        current_idx = end_idx;
        text_start = current_idx;
    }

    if text_start < sql.len() {
        colorize_sql_text(&sql[text_start..], spans);
    }
}

fn colorize_sql_text<'a>(text: &'a str, spans: &mut Vec<Span<'a>>) {
    let mut in_word = false;
    let mut word_start = 0;

    for (i, c) in text.char_indices() {
        let is_ident = c.is_alphanumeric() || c == '_' || c == '.';

        if !in_word && is_ident {
            if i > word_start {
                spans.push(Span::styled(
                    text[word_start..i].to_owned(),
                    Style::default().fg(Color::DarkGray),
                ));
            }
            word_start = i;
            in_word = true;
        } else if in_word && !is_ident {
            let word = &text[word_start..i];
            let is_param = (word != "." && word.chars().all(|ch| ch.is_ascii_digit() || ch == '.'))
                || word.eq_ignore_ascii_case("true")
                || word.eq_ignore_ascii_case("false")
                || word.eq_ignore_ascii_case("null");
            let color = if is_param {
                Color::Red
            } else {
                Color::DarkGray
            };
            spans.push(Span::styled(word.to_owned(), Style::default().fg(color)));
            word_start = i;
            in_word = false;
        }
    }

    if word_start < text.len() {
        if in_word {
            let word = &text[word_start..];
            let is_param = (word != "." && word.chars().all(|ch| ch.is_ascii_digit() || ch == '.'))
                || word.eq_ignore_ascii_case("true")
                || word.eq_ignore_ascii_case("false")
                || word.eq_ignore_ascii_case("null");
            let color = if is_param {
                Color::Red
            } else {
                Color::DarkGray
            };
            spans.push(Span::styled(word.to_owned(), Style::default().fg(color)));
        } else {
            spans.push(Span::styled(
                text[word_start..].to_owned(),
                Style::default().fg(Color::DarkGray),
            ));
        }
    }
}

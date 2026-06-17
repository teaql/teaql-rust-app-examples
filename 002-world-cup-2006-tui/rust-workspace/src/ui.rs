use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Text, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};
use fifa_world_cup_2026_service::E;
use crate::app::{App, View};

fn get_block(title: String, is_active: bool) -> Block<'static> {
    let mut b = Block::default().borders(Borders::ALL).title(title);
    if is_active {
        b = b.border_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
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
        View::Global => "World Cup 2026 - Terminal Dashboard".to_string(),
        View::Group(g) => format!("World Cup 2026 - Terminal Dashboard (Group {})", g),
        View::Players => "World Cup 2026 - Terminal Dashboard (Players)".to_string(),
        View::Logs => "World Cup 2026 - System Logs".to_string(),
    };
    let header = Paragraph::new(header_text)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Main content
    match app.view.clone() {
        View::Global => render_global(f, app, chunks[1]),
        View::Group(ref g) => render_group(f, app, chunks[1], g),
        View::Players => render_players(f, app, chunks[1]),
        View::Logs => render_logs(f, app, chunks[1]),
    }

    // Input area
    let input = Paragraph::new(format!("> {}", app.input_buffer))
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Commands: global | group A | players | sync live | logs | quit"));
    f.render_widget(input, chunks[2]);

    #[allow(deprecated)]
    f.set_cursor_position((
        chunks[2].x + 3 + app.input_buffer.chars().count() as u16,
        chunks[2].y + 1,
    ));
}

fn render_global(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // All teams standings
    let header_cells = ["Grp", "Team", "P", "W", "D", "L", "GF", "GA", "GD", "Pts"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells).style(Style::default().add_modifier(Modifier::BOLD)).height(1).bottom_margin(1);
    
    let rows: Vec<Row> = app.global_standings.iter().map(|s| {
        let t_opt = E::group_standing(s).get_tournament_team().eval();
        let g_opt = E::group_standing(s).get_match_group().eval();
        
        let group_name = g_opt.map(|g| g.group_letter().to_string()).unwrap_or_else(|| "?".to_string());
        let team_display = t_opt.map(|t| format!("{} {}", t.emoji_flag(), t.team_name())).unwrap_or_else(|| "Unknown".to_string());
        
        Row::new(vec![
            Cell::from(group_name),
            Cell::from(team_display),
            Cell::from(s.played().to_string()),
            Cell::from(s.won().to_string()),
            Cell::from(s.drawn().to_string()),
            Cell::from(s.lost().to_string()),
            Cell::from(s.goals_for().to_string()),
            Cell::from(s.goals_against().to_string()),
            Cell::from(s.goal_difference().to_string()),
            Cell::from(s.points().to_string()).style(Style::default().fg(Color::Green)),
        ])
    }).collect();

    let g_len = app.global_standings.len();
    let g_sel = app.global_table_state.selected().unwrap_or(0);
    let title = format!("All Teams Standings  {} / {}  ↑/↓ to scroll", g_sel + 1, g_len);
    let table = Table::new(rows, [
        Constraint::Length(4),
        Constraint::Length(25),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(4),
        Constraint::Length(4),
    ])
    .header(header)
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .block(get_block(title, app.active_pane == 0));
    f.render_stateful_widget(table, left_chunks[0], &mut app.global_table_state);

    // Top Players
    let p_header = Row::new(vec!["#", "Team", "Player", "Goals"]).style(Style::default().add_modifier(Modifier::BOLD)).bottom_margin(1);
    let p_rows: Vec<Row> = app.top_players.iter().enumerate().map(|(i, (t, p, c))| {
        Row::new(vec![
            Cell::from((i + 1).to_string()),
            Cell::from(t.clone()),
            Cell::from(p.clone()),
            Cell::from(c.to_string()).style(Style::default().fg(Color::Yellow)),
        ])
    }).collect();
    
    let p_len = app.top_players.len();
    let p_sel = app.players_state.selected().unwrap_or(0);
    let p_title = format!("Top Players  {} / {}  ↑/↓ to scroll", if p_len > 0 { p_sel + 1 } else { 0 }, p_len);

    let p_table = Table::new(p_rows, [
        Constraint::Length(4),
        Constraint::Percentage(30),
        Constraint::Percentage(50),
        Constraint::Percentage(15),
    ])
    .header(p_header)
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .block(get_block(p_title, app.active_pane == 1));
    f.render_stateful_widget(p_table, right_chunks[0], &mut app.players_state);

    // Completed Matches
    let m_header = Row::new(vec![
        Cell::from(Line::from("Home").alignment(Alignment::Right)),
        Cell::from(Line::from("Score").alignment(Alignment::Center)),
        Cell::from(Line::from("Away").alignment(Alignment::Left))
    ]).style(Style::default().add_modifier(Modifier::BOLD)).bottom_margin(1);
    let m_rows: Vec<Row> = app.recent_matches.iter().map(|m| {
        let h = m.home_team().map(|t| format!("{} {}", t.team_name(), t.emoji_flag())).unwrap_or_default();
        let a = m.away_team().map(|t| format!("{} {}", t.emoji_flag(), t.team_name())).unwrap_or_default();
        let s = format!("{} - {}", m.home_score(), m.away_score());
        Row::new(vec![
            Cell::from(Line::from(h).alignment(Alignment::Right)),
            Cell::from(Line::from(s).alignment(Alignment::Center)),
            Cell::from(Line::from(a).alignment(Alignment::Left))
        ])
    }).collect();
    let m_len = app.recent_matches.len();
    let m_sel = app.matches_state.selected().unwrap_or(0);
    let m_title = format!("Completed Matches  {} / {}  ↑/↓ to scroll", m_sel + 1, m_len);
    let m_table = Table::new(m_rows, [
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(40),
    ])
    .header(m_header)
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .block(get_block(m_title, app.active_pane == 2));
    f.render_stateful_widget(m_table, right_chunks[1], &mut app.matches_state);
}

fn render_group(f: &mut Frame, app: &mut App, area: Rect, g_letter: &str) {

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Group Standings
    let header_cells = ["Team", "P", "W", "D", "L", "GF", "GA", "GD", "Pts"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells).style(Style::default().add_modifier(Modifier::BOLD)).height(1).bottom_margin(1);
    
    let rows: Vec<Row> = app.group_standings.iter().map(|s| {
        let t_opt = E::group_standing(s).get_tournament_team().eval();
        let team_display = t_opt.map(|t| format!("{} {}", t.emoji_flag(), t.team_name())).unwrap_or_else(|| "Unknown".to_string());
        
        Row::new(vec![
            Cell::from(team_display),
            Cell::from(s.played().to_string()),
            Cell::from(s.won().to_string()),
            Cell::from(s.drawn().to_string()),
            Cell::from(s.lost().to_string()),
            Cell::from(s.goals_for().to_string()),
            Cell::from(s.goals_against().to_string()),
            Cell::from(s.goal_difference().to_string()),
            Cell::from(s.points().to_string()).style(Style::default().fg(Color::Green)),
        ])
    }).collect();

    let g_len = app.group_standings.len();
    let g_sel = app.global_table_state.selected().unwrap_or(0);
    let title = format!("Group {} Standings  {} / {}  ↑/↓ to scroll", g_letter, g_sel + 1, g_len);
    let table = Table::new(rows, [
        Constraint::Length(25),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(4),
        Constraint::Length(4),
    ])
    .header(header)
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .block(get_block(title, app.active_pane == 0));
    f.render_stateful_widget(table, chunks[0], &mut app.global_table_state);

    // Group Players
    let p_header = Row::new(vec!["Team", "Player", "Goals"]).style(Style::default().add_modifier(Modifier::BOLD)).bottom_margin(1);
    let p_rows: Vec<Row> = app.group_players.iter().map(|(t, p, c)| {
        Row::new(vec![
            Cell::from(t.clone()),
            Cell::from(p.clone()),
            Cell::from(c.to_string()).style(Style::default().fg(Color::Yellow)),
        ])
    }).collect();
    let p_table = Table::new(p_rows, [
        Constraint::Percentage(30),
        Constraint::Percentage(50),
        Constraint::Percentage(20),
    ])
    .header(p_header)
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .block(get_block(format!("Group {} Top Players", g_letter), app.active_pane == 1));
    f.render_stateful_widget(p_table, right_chunks[0], &mut app.players_state);

    // Group Matches
    let m_header = Row::new(vec![
        Cell::from(Line::from("Home").alignment(Alignment::Right)),
        Cell::from(Line::from("Score").alignment(Alignment::Center)),
        Cell::from(Line::from("Away").alignment(Alignment::Left))
    ]).style(Style::default().add_modifier(Modifier::BOLD)).bottom_margin(1);
    let m_rows: Vec<Row> = app.group_matches.iter().map(|m| {
        let h = m.home_team().map(|t| format!("{} {}", t.team_name(), t.emoji_flag())).unwrap_or_default();
        let a = m.away_team().map(|t| format!("{} {}", t.emoji_flag(), t.team_name())).unwrap_or_default();
        let s = format!("{} - {}", m.home_score(), m.away_score());
        Row::new(vec![
            Cell::from(Line::from(h).alignment(Alignment::Right)),
            Cell::from(Line::from(s).alignment(Alignment::Center)),
            Cell::from(Line::from(a).alignment(Alignment::Left))
        ])
    }).collect();
    let m_len = app.group_matches.len();
    let m_sel = app.matches_state.selected().unwrap_or(0);
    let m_title = format!("Group {} Completed Matches  {} / {}  ↑/↓ to scroll", g_letter, m_sel + 1, m_len);
    let m_table = Table::new(m_rows, [
        Constraint::Percentage(40),
        Constraint::Percentage(20),
        Constraint::Percentage(40),
    ])
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
    ]).style(Style::default().add_modifier(Modifier::BOLD)).bottom_margin(1);

    let rows: Vec<Row> = app.all_players.iter().map(|(t, p, c, m)| {
        let matches_lines: Vec<Line> = m.iter().map(|ms| Line::from(ms.clone())).collect();
        let matches_text = ratatui::text::Text::from(matches_lines);
        let height = m.len().max(1) as u16;

        Row::new(vec![
            Cell::from(t.clone()),
            Cell::from(p.clone()),
            Cell::from(Line::from(c.to_string()).alignment(Alignment::Center)).style(Style::default().fg(Color::Yellow)),
            Cell::from(matches_text),
        ]).height(height)
    }).collect();

    let total = app.all_players.len();
    let sel = app.player_table_state.selected().unwrap_or(0);
    let title = format!("Players  {} / {}  ↑/↓ to scroll", if total > 0 { sel + 1 } else { 0 }, total);

    let table = Table::new(rows, [
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Length(7),
        Constraint::Percentage(50),
    ])
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
    let title = format!("System Logs  {} / {}  ↑/↓ to scroll", if total > 0 { sel + 1 } else { 0 }, total);

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
                spans.push(Span::styled(format!("[{}]", timestamp), Style::default().fg(Color::Indexed(244))));
                rest = &line[time_end+1..];

                // 2. User ID bracket e.g. -[user]
                if rest.starts_with("-[") {
                    if let Some(end) = rest[2..].find(']') {
                        let user_part = &rest[2..end+2];
                        spans.push(Span::styled("-", Style::default().fg(Color::Indexed(240))));
                        spans.push(Span::styled(format!("[{}]", user_part), Style::default().fg(Color::Rgb(155, 89, 182)).add_modifier(Modifier::BOLD)));
                        rest = &rest[end+3..];
                    }
                }

                // 3. Severity Level bracket e.g. -[AUDIT], -[INFO], or -[DEBUG]
                if rest.starts_with("-[") {
                    if let Some(end) = rest[2..].find(']') {
                        let level = &rest[2..end+2];
                        spans.push(Span::styled("-", Style::default().fg(Color::Indexed(240))));
                        if level == "AUDIT" {
                            is_audit = true;
                            spans.push(Span::styled(format!("[{}]", level), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
                        } else if level == "INFO" {
                            spans.push(Span::styled(format!("[{}]", level), Style::default().fg(Color::Rgb(46, 204, 113)).add_modifier(Modifier::BOLD)));
                        } else {
                            spans.push(Span::styled(format!("[{}]", level), Style::default().fg(Color::Indexed(242))));
                        }
                        rest = &rest[end+3..];
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
            let tag = &rest[1..end+1];
            // Highlight µs timing in red, other tags in gray
            let tag_style = if tag.ends_with("µs") {
                Style::default().fg(Color::Rgb(231, 76, 60)).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Indexed(242))
            };
            spans.push(Span::styled(format!("[{}]", tag), tag_style));
            rest = &rest[end+2..];
            if rest.starts_with('-') {
                spans.push(Span::styled("-", Style::default().fg(Color::Indexed(240))));
                rest = &rest[1..];
            }
        }
    }

    // Parse the next bracket too (e.g. [DEBUG] after [µs])
    if rest.starts_with("[") {
        if let Some(end) = rest[1..].find(']') {
            let tag = &rest[1..end+1];
            let tag_style = if tag.ends_with("µs") {
                Style::default().fg(Color::Rgb(231, 76, 60)).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Indexed(242))
            };
            spans.push(Span::styled(format!("[{}]", tag), tag_style));
            rest = &rest[end+2..];
            if rest.starts_with('-') {
                spans.push(Span::styled("-", Style::default().fg(Color::Indexed(240))));
                rest = &rest[1..];
            }
        }
    }

    if rest.starts_with("SqlLogEntry") {
        spans.push(Span::styled("SqlLogEntry", Style::default().fg(Color::Indexed(242))));
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
            let first_segment = &rest[..end+5];
            let after_first = &rest[end+5..];
            
            if after_first.starts_with(" - [") {
                // If there is another " - [" immediately following, then the first one is the comment!
                colorize_comment_segment(first_segment, &mut spans, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
                rest = after_first;
                
                // Now parse the second one as the result summary
                if let Some(end2) = rest[4..].find(']') {
                    let result_part = &rest[..end2+5];
                    colorize_comment_segment(result_part, &mut spans, Style::default().fg(Color::Rgb(52, 152, 219)));
                    rest = &rest[end2+5..];
                }
            } else {
                // If there is no " - [" following, then this first segment is the result summary (no comment exists)!
                colorize_comment_segment(first_segment, &mut spans, Style::default().fg(Color::Rgb(52, 152, 219)));
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
        spans.push(Span::styled("Execute TeaQL - ", Style::default().fg(Color::Indexed(242))));
        spans.push(Span::styled(rest[16..].to_owned(), Style::default().fg(Color::Rgb(46, 204, 113)).add_modifier(Modifier::BOLD)));
    } else if rest.starts_with("Starting business action: ") {
        spans.push(Span::styled("DOMAIN: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
        spans.push(Span::styled(rest[26..].to_owned(), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
    } else if rest.starts_with("Finished business action: ") {
        spans.push(Span::styled("✔ ", Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD)));
        spans.push(Span::styled(rest[26..].to_owned(), Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD)));
    } else if rest.starts_with("Starting query: ") {
        spans.push(Span::styled("▶ ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
        spans.push(Span::styled(rest[16..].to_owned(), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
    } else if rest.starts_with("Finished query: ") {
        spans.push(Span::styled("✔ ", Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD)));
        spans.push(Span::styled(rest[16..].to_owned(), Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD)));
    } else if rest.starts_with("Business Log: ") {
        spans.push(Span::styled("🛈 ", Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD)));
        spans.push(Span::styled(rest[14..].to_owned(), Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD)));
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
            if !inner.is_empty() && (inner.chars().all(|c| c.is_ascii_digit()) || inner == "pending") {
                if abs_start > current_idx {
                    spans.push(Span::styled(&text[current_idx..abs_start + 1], base_style));
                }
                spans.push(Span::styled(inner, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)));
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
        spans.push(Span::styled(sql[abs_quote..end_idx].to_owned(), Style::default().fg(Color::Red)));
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
                spans.push(Span::styled(text[word_start..i].to_owned(), Style::default().fg(Color::DarkGray)));
            }
            word_start = i;
            in_word = true;
        } else if in_word && !is_ident {
            let word = &text[word_start..i];
            let is_param = (word != "." && word.chars().all(|ch| ch.is_ascii_digit() || ch == '.'))
                        || word.eq_ignore_ascii_case("true") 
                        || word.eq_ignore_ascii_case("false") 
                        || word.eq_ignore_ascii_case("null");
            let color = if is_param { Color::Red } else { Color::DarkGray };
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
            let color = if is_param { Color::Red } else { Color::DarkGray };
            spans.push(Span::styled(word.to_owned(), Style::default().fg(color)));
        } else {
            spans.push(Span::styled(text[word_start..].to_owned(), Style::default().fg(Color::DarkGray)));
        }
    }
}

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
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
        View::Team(t) => format!("World Cup 2026 - Terminal Dashboard (Team {})", t),
        View::Player(p) => format!("World Cup 2026 - Terminal Dashboard (Player {})", p),
    };
    let header = Paragraph::new(header_text)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Main content
    // We need mut app to pass table states, but Rust doesn't allow mut app references if we used it earlier.
    // Wait, ui::render(f, &mut app) will be called, so let's change ui::render to take &mut App.
    match app.view.clone() {
        View::Global => render_global(f, app, chunks[1]),
        View::Group(ref g) => render_group(f, app, chunks[1], g),
        View::Player(_) | View::Team(_) => render_player(f, app, chunks[1]),
    }

    // Input area
    let input = Paragraph::new(format!("> {}", app.input_buffer))
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Commands: global | group A | team BRA | player Vinicius | sync live | quit"));
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

    // Recent Matches
    let m_header = Row::new(vec!["Home", "Score", "Away"]).style(Style::default().add_modifier(Modifier::BOLD)).bottom_margin(1);
    let m_rows: Vec<Row> = app.recent_matches.iter().map(|m| {
        let h = m.home_team().map(|t| format!("{} {}", t.emoji_flag(), t.team_name())).unwrap_or_default();
        let a = m.away_team().map(|t| format!("{} {}", t.emoji_flag(), t.team_name())).unwrap_or_default();
        let s = format!("{} - {}", m.home_score(), m.away_score());
        Row::new(vec![Cell::from(h), Cell::from(s), Cell::from(a)])
    }).collect();
    let m_len = app.recent_matches.len();
    let m_sel = app.matches_state.selected().unwrap_or(0);
    let m_title = format!("Recent Matches  {} / {}  ↑/↓ to scroll", m_sel + 1, m_len);
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
    let m_header = Row::new(vec!["Home", "Score", "Away"]).style(Style::default().add_modifier(Modifier::BOLD)).bottom_margin(1);
    let m_rows: Vec<Row> = app.group_matches.iter().map(|m| {
        let h = m.home_team().map(|t| format!("{} {}", t.emoji_flag(), t.team_name())).unwrap_or_default();
        let a = m.away_team().map(|t| format!("{} {}", t.emoji_flag(), t.team_name())).unwrap_or_default();
        let s = format!("{} - {}", m.home_score(), m.away_score());
        Row::new(vec![Cell::from(h), Cell::from(s), Cell::from(a)])
    }).collect();
    let m_len = app.group_matches.len();
    let m_sel = app.matches_state.selected().unwrap_or(0);
    let m_title = format!("Group {} Matches  {} / {}  ↑/↓ to scroll", g_letter, m_sel + 1, m_len);
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

fn render_player(f: &mut Frame, app: &mut App, area: Rect) {
    let header = Row::new(vec!["Team", "Player", "Goals", "Matches"]).style(Style::default().add_modifier(Modifier::BOLD)).bottom_margin(1);
    let rows: Vec<Row> = app.all_players.iter().map(|(t, p, c, m)| {
        Row::new(vec![
            Cell::from(t.clone()),
            Cell::from(p.clone()),
            Cell::from(c.to_string()).style(Style::default().fg(Color::Yellow)),
            Cell::from(m.join(", ")),
        ])
    }).collect();

    let table = Table::new(rows, [
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(10),
        Constraint::Percentage(50),
    ])
    .header(header)
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .block(Block::default().borders(Borders::ALL).title("All Players Ranking"));
    f.render_stateful_widget(table, area, &mut app.player_table_state);
}


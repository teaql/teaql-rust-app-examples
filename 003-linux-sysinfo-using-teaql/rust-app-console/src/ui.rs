use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame,
};

use crate::app::{ActiveList, App, View};

pub fn draw_ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.area());

    // Header
    let sys_title = if let Some(sys) = &app.sys_info {
        format!(" TeaQL Linux SysInfo | {} | CPU: {} | Mem: {}/{} MB ", 
            sys.hostname(), sys.cpu_count(), sys.memory_available_bytes() / 1024 / 1024, sys.memory_total_bytes() / 1024 / 1024)
    } else {
        " TeaQL Linux SysInfo Loading... ".to_string()
    };
    let header = Paragraph::new(sys_title)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    match app.view {
        View::Dashboard(active) => draw_dashboard(f, app, chunks[1], active),
        View::ProcessDetail(_) => draw_detail(f, app, chunks[1]),
    }
}

fn get_border_style(is_active: bool) -> Style {
    if is_active {
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}

fn format_uptime_ch(seconds: i64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    
    if days > 0 {
        format!("{}天{}小时{}分", days, hours, minutes)
    } else if hours > 0 {
        format!("{}小时{}分", hours, minutes)
    } else {
        format!("{}分", minutes)
    }
}

fn draw_dashboard(f: &mut Frame, app: &mut App, area: Rect, active: ActiveList) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

    // Memory Table
    let mem_rows = app.mem_procs.iter().map(|p| {
        Row::new(vec![
            p.pid().to_string(),
            p.name().to_string(),
            format!("{} MB", p.memory_rss_kb() / 1024),
        ])
    });
    let mem_table = Table::new(mem_rows, [Constraint::Length(10), Constraint::Percentage(60), Constraint::Length(15)])
        .header(Row::new(vec!["PID", "Name", "Memory (RSS)"]).style(Style::default().add_modifier(Modifier::BOLD)))
        .block(Block::default().borders(Borders::ALL).title(" Most Memory Usage ").border_style(get_border_style(active == ActiveList::Memory)))
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    f.render_stateful_widget(mem_table, chunks[0], &mut app.mem_state);

    // Uptime Table
    let time_rows = app.time_procs.iter().map(|p| {
        let uptime = chrono::Utc::now().signed_duration_since(p.create_time()).num_seconds();
        Row::new(vec![
            p.pid().to_string(),
            p.name().to_string(),
            format_uptime_ch(uptime),
        ])
    });
    let time_table = Table::new(time_rows, [Constraint::Length(10), Constraint::Percentage(60), Constraint::Length(15)])
        .header(Row::new(vec!["PID", "Name", "Uptime"]).style(Style::default().add_modifier(Modifier::BOLD)))
        .block(Block::default().borders(Borders::ALL).title(" Longest Running ").border_style(get_border_style(active == ActiveList::Uptime)))
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    f.render_stateful_widget(time_table, chunks[1], &mut app.time_state);

    // CPU Table
    let cpu_rows = app.cpu_procs.iter().map(|p| {
        Row::new(vec![
            p.pid().to_string(),
            p.name().to_string(),
            format!("{} ticks", p.cpu_user_ticks()),
        ])
    });
    let cpu_table = Table::new(cpu_rows, [Constraint::Length(10), Constraint::Percentage(60), Constraint::Length(15)])
        .header(Row::new(vec!["PID", "Name", "CPU (User Ticks)"]).style(Style::default().add_modifier(Modifier::BOLD)))
        .block(Block::default().borders(Borders::ALL).title(" Most CPU Usage ").border_style(get_border_style(active == ActiveList::Cpu)))
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    f.render_stateful_widget(cpu_table, chunks[2], &mut app.cpu_state);

    // Threads Table
    let thread_rows = app.thread_procs.iter().map(|p| {
        Row::new(vec![
            p.pid().to_string(),
            p.name().to_string(),
            p.thread_count().to_string(),
        ])
    });
    let thread_table = Table::new(thread_rows, [Constraint::Length(10), Constraint::Percentage(60), Constraint::Length(15)])
        .header(Row::new(vec!["PID", "Name", "Thread Count"]).style(Style::default().add_modifier(Modifier::BOLD)))
        .block(Block::default().borders(Borders::ALL).title(" Most Threads ").border_style(get_border_style(active == ActiveList::Threads)))
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    f.render_stateful_widget(thread_table, chunks[3], &mut app.thread_state);
}

fn draw_detail(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Min(0)])
        .split(area);

    if let Some(p) = &app.detail_proc {
        let uptime = chrono::Utc::now().signed_duration_since(p.create_time()).num_seconds();
        let detail_text = vec![
            Line::from(vec![
                Span::styled("Process: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{} (PID: {})", p.name(), p.pid())),
            ]),
            Line::from(vec![
                Span::styled("Memory (RSS): ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{} MB  |  ", p.memory_rss_kb() / 1024)),
                Span::styled("Memory (VMS): ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{} MB", p.memory_vms_kb() / 1024)),
            ]),
            Line::from(vec![
                Span::styled("CPU (User Ticks): ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{}  |  ", p.cpu_user_ticks())),
                Span::styled("CPU (Sys Ticks): ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{}", p.cpu_system_ticks())),
            ]),
            Line::from(vec![
                Span::styled("Thread Count: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{}  |  ", p.thread_count())),
                Span::styled("Uptime: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format_uptime_ch(uptime)),
            ]),
        ];
        let p_widget = Paragraph::new(detail_text)
            .block(Block::default().borders(Borders::ALL).title(" Process Details ").border_style(Style::default().fg(Color::Cyan)));
        f.render_widget(p_widget, chunks[0]);
    } else {
        let p_widget = Paragraph::new("Loading details...")
            .block(Block::default().borders(Borders::ALL).title(" Process Details "));
        f.render_widget(p_widget, chunks[0]);
    }

    let rows = app.detail_threads.iter().map(|t| {
        Row::new(vec![
            t.tid().to_string(),
            t.name().to_string(),
            t.state().to_string(),
            t.cpu_user_ticks().to_string(),
        ])
    });
    let t_table = Table::new(rows, [Constraint::Length(10), Constraint::Percentage(40), Constraint::Length(10), Constraint::Length(15)])
        .header(Row::new(vec!["TID", "Name", "State", "User Ticks"]).style(Style::default().add_modifier(Modifier::BOLD)))
        .block(Block::default().borders(Borders::ALL).title(" Threads ").border_style(Style::default().fg(Color::Green)))
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    
    f.render_stateful_widget(t_table, chunks[1], &mut app.detail_thread_state);
}

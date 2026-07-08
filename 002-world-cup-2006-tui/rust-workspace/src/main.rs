use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use fifa_world_cup_2026_service::{service_runtime, ServiceRuntimeConfig};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::time::Duration;
use std::{error::Error, io};

mod app;
mod seed;
mod ui;

use app::App;
use std::sync::{Arc, Mutex};
use teaql_runtime::{
    LogPayload, SafeAuditEvent, SafeAuditEventSink, UnifiedLogBuffer, UserContext,
};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(Clone)]
struct LogWriter {
    logs: Arc<Mutex<Vec<String>>>,
}

impl io::Write for LogWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Ok(s) = std::str::from_utf8(buf) {
            let msg = s.trim().to_string();
            if !msg.is_empty() {
                if let Ok(mut logs) = self.logs.lock() {
                    logs.push(msg);
                    if logs.len() > 1000 {
                        logs.remove(0);
                    }
                }
            }
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for LogWriter {
    type Writer = Self;
    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

struct MyLogSink {
    logs: Arc<Mutex<Vec<String>>>,
}

impl SafeAuditEventSink for MyLogSink {
    fn on_safe_event(
        &self,
        _ctx: &UserContext,
        event: &SafeAuditEvent,
    ) -> Result<(), teaql_runtime::RuntimeError> {
        let purpose = event
            .trace_chain
            .first()
            .map(|n| n.comment.as_str())
            .unwrap_or("unknown");
        let msg = format!(
            "TeaQL Audit: Entity={} Purpose={} Kind={:?}",
            event.entity, purpose, event.kind
        );
        let ts = chrono::Local::now().format("%H:%M:%S").to_string();
        if let Ok(mut logs) = self.logs.lock() {
            logs.push(format!("[{}] {}", ts, msg));
            if logs.len() > 500 {
                logs.remove(0);
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let headless = args.contains(&"--headless".to_string());
    let reset = args.contains(&"--reset".to_string());

    let db_path = {
        if let Some(mut path) = dirs::home_dir() {
            path.push(".wc2026");
            if reset {
                let _ = std::fs::remove_dir_all(&path);
            }
            std::fs::create_dir_all(&path)?;
            path.push("worldcup.db");
            path.to_string_lossy().to_string()
        } else {
            "worldcup.db".to_string()
        }
    };

    let config = ServiceRuntimeConfig {
        database_url: format!("sqlite:{}", db_path),
    };

    let logs = Arc::new(Mutex::new(vec!["Application started.".to_string()]));
    let sink = MyLogSink {
        logs: Arc::clone(&logs),
    };

    // Set up tracing subscriber
    let log_writer = LogWriter {
        logs: Arc::clone(&logs),
    };
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,teaql=debug,teaql_provider_sqlite=debug,sqlx=debug,fifa_world_cup_2026_service=debug"));
    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(log_writer)
                .without_time(),
        )
        .try_init();

    let mut ctx = service_runtime(config).await?;
    ctx.set_custom_event_sink(sink);

    // Enable SQL logging through UnifiedLogBuffer
    let log_buffer = UnifiedLogBuffer::default();
    let entries_arc = log_buffer.entries.clone();
    ctx.insert_resource(log_buffer);
    ctx.enable_all_sql_log();

    let logs_for_bg = Arc::clone(&logs);
    tokio::spawn(async move {
        let mut sql_log_index = 0;
        loop {
            if let Ok(entries) = entries_arc.lock() {
                if entries.len() > sql_log_index {
                    for entry in &entries[sql_log_index..] {
                        if let LogPayload::Sql(sql_entry) = &entry.payload {
                            let local_time: chrono::DateTime<chrono::Local> =
                                entry.timestamp.into();
                            let ts = local_time.format("%H:%M:%S%.3f").to_string();
                            let elapsed_us =
                                (sql_entry.elapsed.as_secs_f64() * 1_000_000.0).round() as u64;
                            let trace = if entry.trace_chain.is_empty() {
                                "".to_string()
                            } else {
                                format!(
                                    " - [{}]",
                                    entry
                                        .trace_chain
                                        .iter()
                                        .map(|n| n.comment.clone())
                                        .collect::<Vec<_>>()
                                        .join(" -> ")
                                )
                            };
                            let single_line_sql = sql_entry.debug_sql.replace('\n', " ");
                            let uid = entry
                                .user_identifier
                                .clone()
                                .unwrap_or_else(|| "system".to_string());
                            let line1 = format!(
                                "[{}]-[{}]-[{:>5}µs]-[DEBUG]-SqlLogEntry{} - [{}]",
                                ts, uid, elapsed_us, trace, sql_entry.result_summary
                            );
                            let line2 = format!("          {}", single_line_sql);
                            if let Ok(mut logs) = logs_for_bg.lock() {
                                logs.push(line1);
                                logs.push(line2);
                                while logs.len() > 1000 {
                                    logs.remove(0);
                                }
                            }
                        }
                    }
                    sql_log_index = entries.len();
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    });

    // Seed data if not initialized
    seed::seed_data(&ctx).await?;

    // Headless mode: print bracket data and exit
    if headless {
        let mut app = App::new(ctx, logs);
        // Sync live goal data from internet
        if let Err(e) = app.sync_live_events().await {
            eprintln!("Warning: Could not sync live data: {}", e);
        }
        app.fetch_data().await?;
        print_headless_bracket(&app);
        return Ok(());
    }

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new(ctx, logs);
    // Sync live goal data from internet
    if let Err(e) = app.sync_live_events().await {
        app.log(&format!("Warning: Could not sync live data: {}", e));
    }
    app.fetch_data().await?;
    let res = run_app(&mut terminal, &mut app).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn print_headless_bracket(app: &App) {
    use std::collections::BTreeMap;

    // Show top scorers if available
    if !app.top_players.is_empty() {
        println!();
        println!("⚽ Top Scorers:");
        println!("{:<5} {:<20} {:<20} {}", "#", "Team", "Player", "Goals");
        println!("{}", "─".repeat(65));
        for (i, (team, player, goals)) in app.top_players.iter().take(10).enumerate() {
            println!("{:<5} {:<20} {:<20} {}", i + 1, team, player, goals);
        }
    }

    let matches = &app.knockout_matches;
    if matches.is_empty() {
        println!("No knockout matches found.");
        return;
    }

    // Group by stage_rank, exclude Third Place (5)
    let mut stages: BTreeMap<usize, Vec<&app::KnockoutMatchView>> = BTreeMap::new();
    for m in matches {
        if m.stage_rank != 5 {
            stages.entry(m.stage_rank).or_default().push(m);
        }
    }

    let keys: Vec<usize> = stages.keys().copied().collect();
    let ncols = keys.len();
    let r32 = stages.get(&1).map_or(0, |v| v.len());
    if r32 == 0 {
        println!("No R32 matches found.");
        return;
    }

    let nrows = 2 * r32 - 1;

    // Line positions
    let mut pos: Vec<Vec<usize>> = Vec::new();
    for (si, &key) in keys.iter().enumerate() {
        let cnt = stages[&key].len();
        let stride = 1 << (si + 1);
        let offset = (1 << si) - 1;
        pos.push((0..cnt).map(|i| offset + i * stride).collect());
    }

    // Connectors
    #[derive(Clone, Copy)]
    enum Conn { E, T, M, B, V }
    let mut cx: Vec<Vec<Conn>> = vec![vec![Conn::E; ncols.saturating_sub(1)]; nrows];

    for c in 0..ncols.saturating_sub(1) {
        for (j, &lm) in pos[c + 1].iter().enumerate() {
            let lt = pos[c][2 * j];
            let lb = pos[c][2 * j + 1];
            if lt < nrows { cx[lt][c] = Conn::T; }
            if lm < nrows { cx[lm][c] = Conn::M; }
            if lb < nrows { cx[lb][c] = Conn::B; }
            for l in (lt + 1)..lm { if l < nrows { cx[l][c] = Conn::V; } }
            for l in (lm + 1)..lb { if l < nrows { cx[l][c] = Conn::V; } }
        }
    }

    let col_w = 22usize;

    // Header
    let stage_label = |k: usize| -> &str {
        match k { 1 => "R32", 2 => "R16", 3 => "QF", 4 => "SF", 6 => "Final", _ => "?" }
    };
    let mut hdr = String::new();
    for (i, &k) in keys.iter().enumerate() {
        hdr.push_str(&format!("{:^w$}", stage_label(k), w = col_w));
        if i < ncols - 1 { hdr.push_str("   "); }
    }
    println!("{}", hdr);
    println!("{}", "─".repeat(col_w * ncols + 3 * ncols.saturating_sub(1)));

    // Data rows
    for r in 0..nrows {
        let mut line = String::new();
        for c in 0..ncols {
            let m_opt = pos[c].iter().position(|&p| p == r).map(|i| stages[&keys[c]][i]);
            if let Some(m) = m_opt {
                let h = crate::ui::team_short(&m.home);
                let a = crate::ui::team_short(&m.away);
                let text = format!("{:>6} {} {:<6}", h, m.score, a);
                line.push_str(&format!("{:<w$}", text, w = col_w));
            } else {
                line.push_str(&" ".repeat(col_w));
            }
            if c < ncols - 1 {
                line.push_str(match cx[r][c] {
                    Conn::E => "   ",
                    Conn::T => " ─┐",
                    Conn::M => " ├─",
                    Conn::B => " ─┘",
                    Conn::V => " │ ",
                });
            }
        }
        println!("{}", line);
    }

    // Champion
    let champion = matches.iter().find(|m| m.stage_rank == 6)
        .and_then(|m| m.winner.clone())
        .unwrap_or_else(|| "TBD".to_string());
    println!();
    println!("🏆 Champion: {}", champion);

    // Summary
    let is_projected = matches.iter().all(|m| m.score == "vs");
    let completed = matches.iter().filter(|m| m.completed).count();
    if is_projected {
        println!("📊 Status: Projected Bracket (from standings)");
    } else {
        println!("📊 Status: Knockout Results {}/{} completed", completed, matches.len());
    }
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => {
                        app.process_command().await;
                    }
                    KeyCode::Char(c) => {
                        app.input_buffer.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input_buffer.pop();
                    }
                    KeyCode::Esc => {
                        app.should_quit = true;
                    }
                    KeyCode::Up => {
                        app.previous();
                    }
                    KeyCode::Down => {
                        app.next();
                    }
                    KeyCode::Tab => {
                        app.next_pane();
                    }
                    KeyCode::BackTab => {
                        app.prev_pane();
                    }
                    _ => {}
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

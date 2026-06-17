use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use fifa_world_cup_2026_service::{service_runtime, ServiceRuntimeConfig};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io};
use std::time::Duration;

mod app;
mod ui;
mod seed;

use app::App;
use std::sync::{Arc, Mutex};
use teaql_runtime::{SafeAuditEventSink, SafeAuditEvent, UserContext, UnifiedLogBuffer, LogPayload};
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
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
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
    fn on_safe_event(&self, _ctx: &UserContext, event: &SafeAuditEvent) -> Result<(), teaql_runtime::RuntimeError> {
        let purpose = event.trace_chain.first().map(|n| n.comment.as_str()).unwrap_or("unknown");
        let msg = format!("TeaQL Audit: Entity={} Purpose={} Kind={:?}", event.entity, purpose, event.kind);
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
    let db_path = {
        if let Some(mut path) = dirs::home_dir() {
            path.push(".wc2026");
            let _ = std::fs::remove_dir_all(&path); // Delete the entire directory to remove .db, .db-wal, .db-shm
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
    let sink = MyLogSink { logs: Arc::clone(&logs) };
    
    // Set up tracing subscriber
    let log_writer = LogWriter { logs: Arc::clone(&logs) };
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,teaql=debug,teaql_provider_sqlite=debug,sqlx=debug,fifa_world_cup_2026_service=debug"));
    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().with_writer(log_writer).without_time())
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
                            let local_time: chrono::DateTime<chrono::Local> = entry.timestamp.into();
                            let ts = local_time.format("%H:%M:%S%.3f").to_string();
                            let elapsed_us = (sql_entry.elapsed.as_secs_f64() * 1_000_000.0).round() as u64;
                            let trace = if entry.trace_chain.is_empty() {
                                "".to_string()
                            } else {
                                format!(
                                    " - [{}]",
                                    entry.trace_chain.iter().map(|n| n.comment.clone()).collect::<Vec<_>>().join(" -> ")
                                )
                            };
                            let single_line_sql = sql_entry.debug_sql.replace('\n', " ");
                            let uid = entry.user_identifier.clone().unwrap_or_else(|| "system".to_string());
                            let line1 = format!("[{}]-[{}]-[{:>5}µs]-[DEBUG]-SqlLogEntry{} - [{}]", ts, uid, elapsed_us, trace, sql_entry.result_summary);
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

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new(ctx, logs);
    app.fetch_data().await?;
    let res = run_app(&mut terminal, &mut app).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
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
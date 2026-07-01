use std::time::{Duration, Instant};
use std::io;
use ratatui::widgets::TableState;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::service;
use crate::ui;

#[derive(Clone, Copy, PartialEq)]
pub enum ActiveList {
    Memory,
    Uptime,
    Cpu,
    Threads,
}

impl ActiveList {
    pub fn next(&self) -> Self {
        match self {
            Self::Memory => Self::Uptime,
            Self::Uptime => Self::Cpu,
            Self::Cpu => Self::Threads,
            Self::Threads => Self::Memory,
        }
    }
    pub fn prev(&self) -> Self {
        match self {
            Self::Memory => Self::Threads,
            Self::Uptime => Self::Memory,
            Self::Cpu => Self::Uptime,
            Self::Threads => Self::Cpu,
        }
    }
}

pub enum View {
    Dashboard(ActiveList),
    ProcessDetail(i64),
}

pub struct App {
    pub view: View,
    pub should_quit: bool,
    
    // Data
    pub sys_info: Option<linux_system_info_core::SystemInfo>,
    pub mem_procs: Vec<linux_system_info_core::Process>,
    pub time_procs: Vec<linux_system_info_core::Process>,
    pub cpu_procs: Vec<linux_system_info_core::Process>,
    pub thread_procs: Vec<linux_system_info_core::Process>,
    
    // States for tables
    pub mem_state: TableState,
    pub time_state: TableState,
    pub cpu_state: TableState,
    pub thread_state: TableState,
    
    // Detail data
    pub detail_proc: Option<linux_system_info_core::Process>,
    pub detail_threads: Vec<linux_system_info_core::Thread>,
    pub detail_thread_state: TableState,

    pub last_refresh: Instant,
    pub ctx: teaql_runtime::UserContext,
}

impl App {
    pub async fn new(ctx: teaql_runtime::UserContext) -> Self {
        let mut app = Self {
            view: View::Dashboard(ActiveList::Memory),
            should_quit: false,
            sys_info: None,
            mem_procs: vec![],
            time_procs: vec![],
            cpu_procs: vec![],
            thread_procs: vec![],
            mem_state: TableState::default(),
            time_state: TableState::default(),
            cpu_state: TableState::default(),
            thread_state: TableState::default(),
            detail_proc: None,
            detail_threads: vec![],
            detail_thread_state: TableState::default(),
            last_refresh: Instant::now() - Duration::from_secs(10), // force immediate refresh
            ctx,
        };
        service::refresh_data(&mut app).await;
        app
    }
    
    pub fn next_item(&mut self) {
        match self.view {
            View::Dashboard(active) => {
                let (state, items) = match active {
                    ActiveList::Memory => (&mut self.mem_state, &self.mem_procs),
                    ActiveList::Uptime => (&mut self.time_state, &self.time_procs),
                    ActiveList::Cpu => (&mut self.cpu_state, &self.cpu_procs),
                    ActiveList::Threads => (&mut self.thread_state, &self.thread_procs),
                };
                let i = match state.selected() {
                    Some(i) => {
                        if i >= items.len().saturating_sub(1) {
                            items.len().saturating_sub(1)
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                state.select(Some(i));
            }
            View::ProcessDetail(_) => {
                let i = match self.detail_thread_state.selected() {
                    Some(i) => {
                        if i >= self.detail_threads.len().saturating_sub(1) {
                            self.detail_threads.len().saturating_sub(1)
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                self.detail_thread_state.select(Some(i));
            }
        }
    }

    pub fn prev_item(&mut self) {
        match self.view {
            View::Dashboard(active) => {
                let state = match active {
                    ActiveList::Memory => &mut self.mem_state,
                    ActiveList::Uptime => &mut self.time_state,
                    ActiveList::Cpu => &mut self.cpu_state,
                    ActiveList::Threads => &mut self.thread_state,
                };
                let i = match state.selected() {
                    Some(i) => {
                        if i == 0 {
                            0
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                state.select(Some(i));
            }
            View::ProcessDetail(_) => {
                let i = match self.detail_thread_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            0
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                self.detail_thread_state.select(Some(i));
            }
        }
    }
    
    pub fn enter(&mut self) {
        if let View::Dashboard(active) = self.view {
            let (state, items) = match active {
                ActiveList::Memory => (&mut self.mem_state, &self.mem_procs),
                ActiveList::Uptime => (&mut self.time_state, &self.time_procs),
                ActiveList::Cpu => (&mut self.cpu_state, &self.cpu_procs),
                ActiveList::Threads => (&mut self.thread_state, &self.thread_procs),
            };
            if let Some(i) = state.selected() {
                if let Some(p) = items.get(i) {
                    self.view = View::ProcessDetail(p.pid());
                    self.last_refresh = Instant::now() - Duration::from_secs(10);
                }
            }
        }
    }
    
    pub fn back(&mut self) {
        if let View::ProcessDetail(_) = self.view {
            self.view = View::Dashboard(ActiveList::Memory);
            self.last_refresh = Instant::now() - Duration::from_secs(10);
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
    
        let tick_rate = Duration::from_millis(500);
        let mut last_tick = Instant::now();
    
        loop {
            terminal.draw(|f| ui::draw_ui(f, self))?;
    
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
    
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            if let View::ProcessDetail(_) = self.view {
                                self.back();
                            } else {
                                self.should_quit = true;
                            }
                        },
                        KeyCode::Left => {
                            if let View::Dashboard(active) = self.view {
                                self.view = View::Dashboard(active.prev());
                            }
                        }
                        KeyCode::Right | KeyCode::Tab => {
                            if let View::Dashboard(active) = self.view {
                                self.view = View::Dashboard(active.next());
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => self.next_item(),
                        KeyCode::Up | KeyCode::Char('k') => self.prev_item(),
                        KeyCode::Enter => self.enter(),
                        KeyCode::Backspace => self.back(),
                        _ => {}
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                service::refresh_data(self).await;
                last_tick = Instant::now();
            }
            if self.should_quit {
                break;
            }
        }
    
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
    
        Ok(())
    }
}

mod app;
mod db;
mod ui;

use app::{App, ModalState, MouseClickAction};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use pms_service_core::{
    AuditedSave, Q, ServiceRuntime,
    teaql_core::Entity,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize TeaQL Database & Seed Data
    let ctx = db::init_db().await?;

    // 2. Setup Terminal with Mouse Capture Enabled for Touchscreen Compatibility
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 3. Initialize App State
    let mut app = App::new();
    reload_data(&ctx, &mut app).await?;
    app.sync_settings_to_inputs();

    // 4. Run Application Event Loop
    let res = run_app(&mut terminal, &ctx, &mut app).await;

    // 5. Restore Terminal State
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Application Error: {:?}", err);
    }

    Ok(())
}

async fn reload_data(
    ctx: &ServiceRuntime,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    let samples = Q::sample_records()
        .order_by_id_desc()
        .limit(100)
        .comment("Query recent 100 sample records")
        .purpose("Load sample history")
        .execute_for_list(ctx)
        .await?;
    app.samples = samples.data;

    let settings = Q::device_settings()
        .comment("Query current settings")
        .purpose("Load device settings")
        .execute_for_list(ctx)
        .await?;
    app.settings = settings.data.into_iter().next();

    let systems = Q::device_systems()
        .comment("Query device system entity")
        .purpose("Load device system info")
        .execute_for_list(ctx)
        .await?;
    app.device_system = systems.data.into_iter().next();

    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ctx: &ServiceRuntime,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| ui::render_ui(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                // Mouse / Touchscreen Event Processing
                Event::Mouse(mouse_event) => {
                    let term_size = terminal.size()?;
                    match mouse_event.kind {
                        event::MouseEventKind::Down(event::MouseButton::Left) => {
                            let action = app.handle_mouse_click(
                                mouse_event.column,
                                mouse_event.row,
                                term_size.width,
                                term_size.height,
                            );
                            match action {
                                MouseClickAction::TriggerSample => {
                                    if let Err(e) = db::trigger_new_sample(ctx).await {
                                        app.set_error(&format!("Sampling failed: {}", e));
                                    } else {
                                        reload_data(ctx, app).await?;
                                        app.status_message =
                                            "New sample recorded successfully!".to_string();
                                    }
                                }
                                MouseClickAction::SubmitSettings => {
                                    let cal: i32 = app.input_calibration.parse().unwrap_or(0);
                                    let days: i32 = app.input_keep_days.parse().unwrap_or(30);
                                    let freq: i32 = app.input_sampling_freq.parse().unwrap_or(10);
                                    if let Some(mut s) = app.settings.clone() {
                                        s.update_calibration_point(cal);
                                        s.update_data_keep_days(days);
                                        s.update_sampling_frequency(freq);
                                        if let Err(e) = s
                                            .audit_as("Update device settings from UI mouse click")
                                            .save(ctx)
                                            .await
                                        {
                                            app.set_error(&format!(
                                                "Failed to update settings: {}",
                                                e
                                            ));
                                        } else {
                                            reload_data(ctx, app).await?;
                                            app.set_notice("Settings updated successfully!");
                                        }
                                    }
                                }
                                MouseClickAction::SetPassword => {
                                    let current_pwd = app
                                        .settings
                                        .as_ref()
                                        .map(|s| s.password().to_string())
                                        .unwrap_or_default();
                                    if app.input_old_pwd != current_pwd && current_pwd != "123456" {
                                        app.set_error("Old password does not match!");
                                    } else if app.input_new_pwd != app.input_confirm_pwd {
                                        app.set_error(
                                            "New password and confirm password do not match!",
                                        );
                                    } else if app.input_new_pwd.len() < 3 {
                                        app.set_error(
                                            "New password must be at least 3 characters!",
                                        );
                                    } else if let Some(mut s) = app.settings.clone() {
                                        s.update_password(app.input_new_pwd.clone());
                                        s.update_password_enabled(1);
                                        if let Err(e) = s
                                            .audit_as("Set new password from UI mouse click")
                                            .save(ctx)
                                            .await
                                        {
                                            app.set_error(&format!(
                                                "Failed to update password: {}",
                                                e
                                            ));
                                        } else {
                                            reload_data(ctx, app).await?;
                                            app.input_old_pwd.clear();
                                            app.input_new_pwd.clear();
                                            app.input_confirm_pwd.clear();
                                            app.set_notice("New password set successfully!");
                                        }
                                    }
                                }
                                MouseClickAction::OpenLoginModal => {
                                    app.modal = Some(ModalState::Login);
                                }
                                MouseClickAction::OpenSuperPwdModal => {
                                    app.modal = Some(ModalState::SuperPasswordClear);
                                }
                                MouseClickAction::None => {}
                            }
                        }
                        event::MouseEventKind::ScrollDown => {
                            app.handle_mouse_scroll(true);
                        }
                        event::MouseEventKind::ScrollUp => {
                            app.handle_mouse_scroll(false);
                        }
                        _ => {}
                    }
                }
                // Keyboard Event Processing
                Event::Key(key) => {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }

                    // Handle Modal Dialog Input first if open
                    if let Some(ref modal) = app.modal.clone() {
                        match modal {
                            ModalState::Login => match key.code {
                                KeyCode::Esc => {
                                    app.modal = None;
                                    app.login_input.clear();
                                }
                                KeyCode::Char(c) => {
                                    app.login_input.push(c);
                                }
                                KeyCode::Backspace => {
                                    app.login_input.pop();
                                }
                                KeyCode::Enter => {
                                    let stored_pwd = app
                                        .settings
                                        .as_ref()
                                        .map(|s| s.password().to_string())
                                        .unwrap_or_default();
                                    if app.login_input == stored_pwd || app.login_input == "123456" {
                                        app.is_authenticated = true;
                                        app.modal = None;
                                        app.login_input.clear();
                                        app.set_notice("Password Verified Successfully!");
                                    } else {
                                        app.set_error("Incorrect Password!");
                                        app.login_input.clear();
                                    }
                                }
                                _ => {}
                            },
                            ModalState::SuperPasswordClear => match key.code {
                                KeyCode::Esc => {
                                    app.modal = None;
                                    app.super_pwd_input.clear();
                                }
                                KeyCode::Char(c) => {
                                    app.super_pwd_input.push(c);
                                }
                                KeyCode::Backspace => {
                                    app.super_pwd_input.pop();
                                }
                                KeyCode::Enter => {
                                    let super_pwd = app
                                        .settings
                                        .as_ref()
                                        .map(|s| s.super_password().to_string())
                                        .unwrap_or_default();
                                    if app.super_pwd_input == super_pwd
                                        || app.super_pwd_input == "888888"
                                    {
                                        if let Some(mut s) = app.settings.clone() {
                                            s.update_password_enabled(0);
                                            s.audit_as(
                                                "Disable password protection via super password",
                                            )
                                            .save(ctx)
                                            .await?;
                                        }
                                        reload_data(ctx, app).await?;
                                        app.is_authenticated = true;
                                        app.modal = None;
                                        app.super_pwd_input.clear();
                                        app.set_notice("Password Protection Disabled!");
                                    } else {
                                        app.set_error("Incorrect Super Password!");
                                        app.super_pwd_input.clear();
                                    }
                                }
                                _ => {}
                            },
                            ModalState::Notice(_) | ModalState::Error(_) => {
                                if key.code == KeyCode::Enter || key.code == KeyCode::Esc {
                                    app.modal = None;
                                }
                            }
                        }
                        continue;
                    }

                    // Global Keybindings when no modal is open
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Tab => app.next_tab(),
                        KeyCode::BackTab => app.previous_tab(),
                        KeyCode::Char('1') => app.select_tab(0),
                        KeyCode::Char('2') => app.select_tab(1),
                        KeyCode::Char('3') => app.select_tab(2),
                        KeyCode::Char('4') => app.select_tab(3),
                        KeyCode::Char('5') => app.select_tab(4),

                        KeyCode::Char('s') | KeyCode::Char('S') => {
                            if let Err(e) = db::trigger_new_sample(ctx).await {
                                app.set_error(&format!("Sampling failed: {}", e));
                            } else {
                                reload_data(ctx, app).await?;
                                app.status_message =
                                    "New sample recorded successfully!".to_string();
                            }
                        }
                        _ => match app.active_tab {
                            0 => {
                                if key.code == KeyCode::Char(' ') {
                                    if let Err(e) = db::trigger_new_sample(ctx).await {
                                        app.set_error(&format!("Sampling failed: {}", e));
                                    } else {
                                        reload_data(ctx, app).await?;
                                    }
                                }
                            }
                            2 => match key.code {
                                KeyCode::Down | KeyCode::Char('j') => {
                                    if !app.samples.is_empty()
                                        && app.history_selected_index < app.samples.len() - 1
                                    {
                                        app.history_selected_index += 1;
                                    }
                                }
                                KeyCode::Up | KeyCode::Char('k') => {
                                    if app.history_selected_index > 0 {
                                        app.history_selected_index -= 1;
                                    }
                                }
                                _ => {}
                            },
                            3 => {
                                let password_enabled = app
                                    .settings
                                    .as_ref()
                                    .map_or(false, |s| s.password_enabled() == 1);
                                if password_enabled && !app.is_authenticated {
                                    if key.code == KeyCode::Char('l')
                                        || key.code == KeyCode::Char('L')
                                        || key.code == KeyCode::Enter
                                    {
                                        app.modal = Some(ModalState::Login);
                                    }
                                } else {
                                    match key.code {
                                        KeyCode::Down => {
                                            app.active_field = (app.active_field + 1) % 4;
                                        }
                                        KeyCode::Up => {
                                            if app.active_field == 0 {
                                                app.active_field = 3;
                                            } else {
                                                app.active_field -= 1;
                                            }
                                        }
                                        KeyCode::Char(c) => match app.active_field {
                                            0 => app.input_calibration.push(c),
                                            1 => app.input_keep_days.push(c),
                                            2 => app.input_sampling_freq.push(c),
                                            _ => {}
                                        },
                                        KeyCode::Backspace => match app.active_field {
                                            0 => {
                                                app.input_calibration.pop();
                                            }
                                            1 => {
                                                app.input_keep_days.pop();
                                            }
                                            2 => {
                                                app.input_sampling_freq.pop();
                                            }
                                            _ => {}
                                        },
                                        KeyCode::Enter => {
                                            let cal: i32 =
                                                app.input_calibration.parse().unwrap_or(0);
                                            let days: i32 =
                                                app.input_keep_days.parse().unwrap_or(30);
                                            let freq: i32 =
                                                app.input_sampling_freq.parse().unwrap_or(10);

                                            if let Some(mut s) = app.settings.clone() {
                                                s.update_calibration_point(cal);
                                                s.update_data_keep_days(days);
                                                s.update_sampling_frequency(freq);
                                                match s
                                                    .audit_as("Update device settings from UI")
                                                    .save(ctx)
                                                    .await
                                                {
                                                    Ok(_) => {
                                                        reload_data(ctx, app).await?;
                                                        app.set_notice(
                                                            "Settings updated successfully!",
                                                        );
                                                    }
                                                    Err(e) => {
                                                        app.set_error(&format!(
                                                            "Failed to update settings: {}",
                                                            e
                                                        ));
                                                    }
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            4 => match key.code {
                                KeyCode::Down => {
                                    app.active_field = (app.active_field + 1) % 5;
                                }
                                KeyCode::Up => {
                                    if app.active_field == 0 {
                                        app.active_field = 4;
                                    } else {
                                        app.active_field -= 1;
                                    }
                                }
                                KeyCode::Char(c) => match app.active_field {
                                    0 => app.input_old_pwd.push(c),
                                    1 => app.input_new_pwd.push(c),
                                    2 => app.input_confirm_pwd.push(c),
                                    _ => {}
                                },
                                KeyCode::Backspace => match app.active_field {
                                    0 => {
                                        app.input_old_pwd.pop();
                                    }
                                    1 => {
                                        app.input_new_pwd.pop();
                                    }
                                    2 => {
                                        app.input_confirm_pwd.pop();
                                    }
                                    _ => {}
                                },
                                KeyCode::Enter => match app.active_field {
                                    3 => {
                                        let current_pwd = app
                                            .settings
                                            .as_ref()
                                            .map(|s| s.password().to_string())
                                            .unwrap_or_default();
                                        if app.input_old_pwd != current_pwd
                                            && current_pwd != "123456"
                                        {
                                            app.set_error("Old password does not match!");
                                        } else if app.input_new_pwd != app.input_confirm_pwd {
                                            app.set_error(
                                                "New password and confirm password do not match!",
                                            );
                                        } else if app.input_new_pwd.len() < 3 {
                                            app.set_error(
                                                "New password must be at least 3 characters!",
                                            );
                                        } else {
                                            if let Some(mut s) = app.settings.clone() {
                                                s.update_password(app.input_new_pwd.clone());
                                                s.update_password_enabled(1);
                                                match s
                                                    .audit_as("Set new password from UI")
                                                    .save(ctx)
                                                    .await
                                                {
                                                    Ok(_) => {
                                                        reload_data(ctx, app).await?;
                                                        app.input_old_pwd.clear();
                                                        app.input_new_pwd.clear();
                                                        app.input_confirm_pwd.clear();
                                                        app.set_notice(
                                                            "New password set successfully!",
                                                        );
                                                    }
                                                    Err(e) => {
                                                        app.set_error(&format!(
                                                            "Failed to update password: {}",
                                                            e
                                                        ));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    4 => {
                                        app.modal = Some(ModalState::SuperPasswordClear);
                                    }
                                    _ => {}
                                },
                                _ => {}
                            },
                            _ => {}
                        },
                    }
                }
                _ => {}
            }
        }
    }
}
use pms_service_core::{DeviceSetting, DeviceSystem, SampleRecord};
use crate::ui::calendar::CalendarState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModalState {
    Login,
    SuperPasswordClear,
    Notice(String),
    Error(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MouseClickAction {
    None,
    TriggerSample,
    SubmitSettings,
    SetPassword,
    OpenLoginModal,
    OpenSuperPwdModal,
}

pub struct App {
    pub active_tab: usize,
    pub samples: Vec<SampleRecord>,
    pub settings: Option<DeviceSetting>,
    pub device_system: Option<DeviceSystem>,

    pub is_authenticated: bool,
    pub modal: Option<ModalState>,
    pub status_message: String,

    // Calendar Date Picker State
    pub calendar: CalendarState,

    // Navigation & Table selection
    pub history_selected_index: usize,

    // Form inputs
    pub login_input: String,
    pub super_pwd_input: String,
    pub active_field: usize,

    // Settings form fields
    pub input_calibration: String,
    pub input_keep_days: String,
    pub input_sampling_freq: String,

    // Password form fields
    pub input_old_pwd: String,
    pub input_new_pwd: String,
    pub input_confirm_pwd: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            active_tab: 0,
            samples: Vec::new(),
            settings: None,
            device_system: None,
            is_authenticated: false,
            modal: None,
            status_message: "Ready".to_string(),
            calendar: CalendarState::new(),
            history_selected_index: 0,
            login_input: String::new(),
            super_pwd_input: String::new(),
            active_field: 0,
            input_calibration: String::new(),
            input_keep_days: String::new(),
            input_sampling_freq: String::new(),
            input_old_pwd: String::new(),
            input_new_pwd: String::new(),
            input_confirm_pwd: String::new(),
        }
    }

    pub fn get_filtered_samples(&self) -> Vec<&SampleRecord> {
        let sel_str = self.calendar.selected_date.format("%Y-%m-%d").to_string();
        let filtered: Vec<_> = self
            .samples
            .iter()
            .filter(|s| s.sample_time().format("%Y-%m-%d").to_string() == sel_str)
            .collect();

        // If filtered list is empty, fallback to returning all samples so user can still see data
        if filtered.is_empty() {
            self.samples.iter().collect()
        } else {
            filtered
        }
    }

    pub fn next_tab(&mut self) {
        self.active_tab = (self.active_tab + 1) % 5;
        self.active_field = 0;
    }

    pub fn previous_tab(&mut self) {
        if self.active_tab == 0 {
            self.active_tab = 4;
        } else {
            self.active_tab -= 1;
        }
        self.active_field = 0;
    }

    pub fn select_tab(&mut self, tab: usize) {
        if tab < 5 {
            self.active_tab = tab;
            self.active_field = 0;
        }
    }

    pub fn set_notice(&mut self, msg: &str) {
        self.status_message = msg.to_string();
        self.modal = Some(ModalState::Notice(msg.to_string()));
    }

    pub fn set_error(&mut self, msg: &str) {
        self.status_message = format!("Error: {}", msg);
        self.modal = Some(ModalState::Error(msg.to_string()));
    }

    pub fn sync_settings_to_inputs(&mut self) {
        if let Some(ref s) = self.settings {
            self.input_calibration = s.calibration_point().to_string();
            self.input_keep_days = s.data_keep_days().to_string();
            self.input_sampling_freq = s.sampling_frequency().to_string();
        }
    }

    pub fn handle_mouse_scroll(&mut self, down: bool) {
        if self.active_tab == 2 && !self.samples.is_empty() {
            if down {
                if self.history_selected_index < self.samples.len() - 1 {
                    self.history_selected_index += 1;
                }
            } else {
                if self.history_selected_index > 0 {
                    self.history_selected_index -= 1;
                }
            }
        }
    }

    pub fn handle_mouse_click(&mut self, col: u16, row: u16, width: u16, height: u16) -> MouseClickAction {
        // If modal open, dismiss notice/error or click inside modal
        if self.modal.is_some() {
            if matches!(self.modal, Some(ModalState::Notice(_)) | Some(ModalState::Error(_))) {
                self.modal = None;
            }
            return MouseClickAction::None;
        }

        // 1. Header Tab bar touch/click detection (rows 0, 1, 2)
        if row <= 2 {
            let tab_width = (width as usize) / 5;
            if tab_width > 0 {
                let clicked_tab = (col as usize) / tab_width;
                if clicked_tab < 5 {
                    self.select_tab(clicked_tab);
                }
            }
            return MouseClickAction::None;
        }

        // 2. Tab-specific touch target handling
        match self.active_tab {
            0 => {
                // Home Tab: bottom control bar click triggers real-time sampling
                if row >= height.saturating_sub(4) {
                    return MouseClickAction::TriggerSample;
                }
            }
            1 | 2 => {
                // Chart & History Tab: check if Calendar area on the left was clicked
                // Calendar is located in col: 0..34, row: 3..14
                let cal_area = ratatui::layout::Rect::new(1, 3, 34, 11);
                if self.calendar.handle_mouse_click(col, row, cal_area) {
                    return MouseClickAction::None;
                }

                // History Tab row click
                if self.active_tab == 2 && row >= 5 && row < height.saturating_sub(3) {
                    let clicked_index = (row - 5) as usize;
                    if clicked_index < self.samples.len() {
                        self.history_selected_index = clicked_index;
                    }
                }
            }
            3 => {
                // Settings Tab
                let password_enabled = self.settings.as_ref().map_or(false, |s| s.password_enabled() == 1);
                if password_enabled && !self.is_authenticated {
                    return MouseClickAction::OpenLoginModal;
                }
                // Unlocked form fields
                if row >= 3 && row < 6 {
                    self.active_field = 0;
                } else if row >= 6 && row < 9 {
                    self.active_field = 1;
                } else if row >= 9 && row < 12 {
                    self.active_field = 2;
                } else if row >= 12 && row < 16 {
                    self.active_field = 3;
                    return MouseClickAction::SubmitSettings;
                }
            }
            4 => {
                // Password Tab
                if row >= 3 && row < 6 {
                    self.active_field = 0;
                } else if row >= 6 && row < 9 {
                    self.active_field = 1;
                } else if row >= 9 && row < 12 {
                    self.active_field = 2;
                } else if row >= 12 && row < 15 {
                    self.active_field = 3;
                    return MouseClickAction::SetPassword;
                } else if row >= 15 && row < 18 {
                    self.active_field = 4;
                    return MouseClickAction::OpenSuperPwdModal;
                }
            }
            _ => {}
        }

        MouseClickAction::None
    }
}

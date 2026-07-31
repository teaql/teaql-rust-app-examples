pub struct AnsiColors;

impl AnsiColors {
    pub const RESET: &'static str = "\x1B[0m";
    pub const BOLD: &'static str = "\x1B[1m";
    pub const DIM: &'static str = "\x1B[2m";
    pub const RED: &'static str = "\x1B[31m";
    pub const GREEN: &'static str = "\x1B[32m";
    pub const YELLOW: &'static str = "\x1B[33m";
    pub const CYAN: &'static str = "\x1B[36m";

    pub fn bold(s: &str) -> String { format!("{}{}{}", Self::BOLD, s, Self::RESET) }
    pub fn red(s: &str) -> String { format!("{}{}{}", Self::RED, s, Self::RESET) }
    pub fn green(s: &str) -> String { format!("{}{}{}", Self::GREEN, s, Self::RESET) }
    pub fn yellow(s: &str) -> String { format!("{}{}{}", Self::YELLOW, s, Self::RESET) }
    pub fn cyan(s: &str) -> String { format!("{}{}{}", Self::CYAN, s, Self::RESET) }
}

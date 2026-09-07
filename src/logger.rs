use std::fmt::Display;

#[derive(Clone)]
pub struct Logger;

// Probably gonna change it later
impl Logger {
    pub fn new() -> Self {
        Self
    }

    pub fn info(&self, msg: impl Display) {
        println!("      ClientLogger \x1b[32m[ INFO ]\x1b[0m {}", msg); // Green Color
    }

    pub fn warn(&self, msg: impl Display) {
        println!("      ClientLogger \x1b[33m[ WARN ]\x1b[0m {}", msg); // Yellow Color
    }

    pub fn error(&self, msg: impl Display) {
        println!("      ClientLogger \x1b[31m[ ERROR ]\x1b[0m {}", msg); // Red Color
    }

    pub fn debug(&self, msg: impl Display) {
        println!("      ClientLogger \x1b[97;44m[ DEBUG ]\x1b[0m {}", msg); // White blue bgcol
    }

    pub fn critical(&self, msg: impl Display) {
        println!(
            "      ClientLogger \x1b[1;97;41m[ CRITICAL ]\x1b[0m {}",
            msg
        ); // White red bgcol
    }
}

// Macro rules logger formatting
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        println!("      ClientLogger \x1b[32m[ INFO ]\x1b[0m {}", format_args!($($arg)*)); // Green Color
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        println!("      ClientLogger \x1b[33m[ WARN ]\x1b[0m {}", format_args!($($arg)*)); // Yellow Color
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        println!("      ClientLogger \x1b[31m[ ERROR ]\x1b[0m {}", format_args!($($arg)*)); // Red Color
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        println!("      ClientLogger \x1b[97;44m[ DEBUG ]\x1b[0m {}", format_args!($($arg)*)); // White blue bgcol
    };
}

#[macro_export]
macro_rules! log_critical {
    ($($arg:tt)*) => {
        println!("      ClientLogger \x1b[1;97;41m[ CRITICAL ]\x1b[0m {}", format_args!($($arg)*)); // White red bgcol
    };
}

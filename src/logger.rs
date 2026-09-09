use std::sync::Mutex;

use chrono::Local;
use sysinfo::{Pid, ProcessesToUpdate, RefreshKind, System};

static SYSTEM_MONITOR: Mutex<Option<(System, Pid)>> = Mutex::new(None);

pub fn log_header() -> String {
    let time_str = Local::now().format("%I:%M:%S %p");

    let memory_mb = if let Ok(mut guard) = SYSTEM_MONITOR.lock() {
        let (sys, pid) = guard.get_or_insert_with(|| {
            let s = System::new_with_specifics(RefreshKind::nothing());
            let p = Pid::from_u32(std::process::id());

            (s, p)
        });

        sys.refresh_processes(ProcessesToUpdate::Some(&[*pid]), false);

        if let Some(proc) = sys.process(*pid) {
            proc.memory() / 1024 / 1024
        } else {
            0
        }
    } else {
        0
    };

    format!(
        "           \x1b[1;97;33m[ {} : {}mb ]\x1b[0m ClientLogger",
        time_str, memory_mb
    )
}

// Macro rules logger formatting
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        println!("{} \x1b[32m[ INFO ]\x1b[0m {}", logger::log_header(), format_args!($($arg)*)); // Green Color
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        println!("{} \x1b[33m[ WARN ]\x1b[0m {}", logger::log_header(), format_args!($($arg)*)); // Yellow Color
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        println!("{} \x1b[31m[ ERROR ]\x1b[0m {}", $crate::logger::log_header(), format_args!($($arg)*)); // Red Color
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        println!("{} \x1b[97;44m[ DEBUG ]\x1b[0m {}", logger::log_header(), format_args!($($arg)*)); // White blue bgcol
    };
}

#[macro_export]
macro_rules! log_critical {
    ($($arg:tt)*) => {
        println!("{} \x1b[1;97;41m[ CRITICAL ]\x1b[0m {}", $crate::logger::log_header(), format_args!($($arg)*)); // White red bgcol
    };
}

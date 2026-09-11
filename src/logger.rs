use chrono::Local;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use sysinfo::{Pid, ProcessesToUpdate, RefreshKind, System};
use tokio::time::interval;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;

// ==============================================================================
//
//                  Yeah yeah customized logging or type shi
//
// ==============================================================================
//
// Example:
// [11-09-2026 09:42:13 +07:00 : 123 MB] ClientLogger [ INFO | bot::client ]: Hello world
//
// OR ERROR:
// [11-09-2026 09:42:13 +07:00 : 123 MB] ClientLogger [ ERROR | bot::client @ bot.rs:136 ]: Hello world

pub static CURRENT_MEM_MB: AtomicUsize = AtomicUsize::new(0);

pub fn spawn_memory_updater() {
    tokio::spawn(async move {
        let mut sys = System::new_with_specifics(RefreshKind::nothing());

        let pid = Pid::from_u32(std::process::id());

        let mut ticker = interval(Duration::from_secs(2));

        loop {
            ticker.tick().await;

            sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), false);

            if let Some(proc) = sys.process(pid) {
                let mb = (proc.memory() / 1024 / 1024) as usize;

                CURRENT_MEM_MB.store(mb, Ordering::Relaxed);
            }
        }
    });
}

// Custom Formatter used by tracing_subscriber
pub struct CustomLogFormatter;

impl<S, N> FormatEvent<S, N> for CustomLogFormatter
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> fmt::Result {
        let now = Local::now();

        let time_str = now.format("%d-%m-%Y %H:%M:%S %:z");

        let mem = CURRENT_MEM_MB.load(Ordering::Relaxed);

        let metadata = event.metadata();

        let level = metadata.level();
        let target = metadata.target();

        let file = metadata.file();
        let line = metadata.line();

        let level_str = match *level {
            tracing::Level::ERROR => "\x1b[97;41mERROR\x1b[0m",
            tracing::Level::WARN => "\x1b[97;43mWARN\x1b[0m",
            tracing::Level::INFO => "\x1b[97;42mINFO\x1b[0m",
            tracing::Level::DEBUG => "\x1b[97;44mDEBUG\x1b[0m",
            tracing::Level::TRACE => "\x1b[97;45mTRACE\x1b[0m",
        };

        let show_loc = matches!(*level, tracing::Level::WARN | tracing::Level::ERROR);

        write!(
            writer,
            "           \x1b[1;97;33m[ {} : {} MB ] \x1b[1;97mClientLogger\x1b[0m [ {} | {}",
            time_str, mem, level_str, target
        )?;

        if show_loc {
            if let Some(file) = file {
                write!(writer, " @ {}", file)?;
            }

            if let Some(line) = line {
                write!(writer, " : {}", line)?;
            }
        }

        write!(writer, " ]:")?;

        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

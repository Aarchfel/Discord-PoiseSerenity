use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use sysinfo::{Pid, ProcessesToUpdate, RefreshKind, System};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;

// ==============================================================================
//
//                  Yeah yeah customized logging or type shi
//
// ==============================================================================

pub static CURRENT_MEM_MB: AtomicUsize = AtomicUsize::new(0);

pub fn spawn_memory_updater() {
    tokio::spawn(async move {
        let mut sys = System::new_with_specifics(RefreshKind::nothing());
        let pid = Pid::from_u32(std::process::id());

        loop {
            sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), false);
            if let Some(proc) = sys.process(pid) {
                let mb = (proc.memory() / 1024 / 1024) as usize;
                CURRENT_MEM_MB.store(mb, Ordering::Relaxed);
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    });
}

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
        let time_str = chrono::Local::now().format("%I:%M:%S %p");
        let mem = CURRENT_MEM_MB.load(Ordering::Relaxed);
        let level = event.metadata().level();

        let level_str = match *level {
            tracing::Level::ERROR => "\x1b[31m[ ERROR ]\x1b[0m",
            tracing::Level::WARN => "\x1b[33m[ WARN ]\x1b[0m",
            tracing::Level::INFO => "\x1b[32m[ INFO ]\x1b[0m",
            tracing::Level::DEBUG => "\x1b[97;44m[ DEBUG ]\x1b[0m",
            tracing::Level::TRACE => "\x1b[35m[ TRACE ]\x1b[0m",
        };

        write!(
            writer,
            "           \x1b[1;97;33m[ {} : {}mb ]\x1b[0m {} ",
            time_str, mem, level_str
        )?;

        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

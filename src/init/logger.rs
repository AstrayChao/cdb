use chrono::Local;
use config::Config;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

pub fn init_logger(cfg: Option<&Config>) -> WorkerGuard {
    let (level, (non_blocking, guard)) = match cfg {
        Some(cfg) => {
            let level = if cfg.get_bool("app.debug").unwrap_or_default() {
                Level::DEBUG
            } else {
                Level::INFO
            };

            let appender = if cfg.get_string("app.env")
                .unwrap_or(String::from("dev")) == "dev" {
                tracing_appender::non_blocking(std::io::stdout())
            } else {
                tracing_appender::non_blocking(tracing_appender::rolling::daily(
                    cfg.get_string("log.path").unwrap_or(String::from("logs")),
                    cfg.get_string("log.filename")
                        .unwrap_or(String::from("log.log")),
                ))
            };

            (level, appender)
        }
        None => (
            Level::DEBUG,
            tracing_appender::non_blocking(tracing_appender::rolling::daily("logs", "log.log")),
        ),
    };

    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_file(true)
        .with_line_number(true)
        .with_ansi(true)
        .with_timer(LocalTimer)
        .with_writer(non_blocking)
        .json()
        .flatten_event(true)
        .init();

    guard
}
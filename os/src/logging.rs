use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

struct SimpleLogger;

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| {
        log::set_max_level(match option_env!("LOG") {
            Some("error") => LevelFilter::Error,
            Some("ERROR") => LevelFilter::Error,
            Some("warn") => LevelFilter::Warn,
            Some("WARN") => LevelFilter::Warn,
            Some("info") => LevelFilter::Info,
            Some("INFO") => LevelFilter::Info,
            Some("debug") => LevelFilter::Debug,
            Some("DEBUG") => LevelFilter::Debug,
            Some("trace") => LevelFilter::Trace,
            Some("TRACE") => LevelFilter::Trace,
            _ => LevelFilter::Off,
        })
    })
}

impl log::Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        // metadata.level() <= Level::Info
        return true;
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        println!(
            "\x1b[{}m{} - {}\x1b[0m",
            map(record.level()),
            record.level(),
            record.args()
        );
    }

    fn flush(&self) {}
}

fn map(level: Level) -> usize {
    match level {
        Level::Error => 31,
        Level::Warn => 93,
        Level::Info => 34,
        Level::Debug => 32,
        Level::Trace => 90,
    }
}

extern crate log;


use log::{LogRecord, LogLevel, LogMetadata, Log, LogLevelFilter};

struct SimpleLogger {
    level: LogLevel,
}

impl SimpleLogger {
    fn new(level: LogLevel) -> SimpleLogger {
        SimpleLogger { level: level }
    }
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{}	{}	{}", record.target(), record.level(), record.args());
        }
    }
}

pub fn init(level: LogLevel) {
    match log::set_logger(|level_holder| {
        level_holder.set(LogLevelFilter::Info);
        Box::new(SimpleLogger::new(level))
    }) {
        Ok(_) => {}
        Err(e) => {
            println!("unable to init logger due to error {}", e);
        }
    }
}

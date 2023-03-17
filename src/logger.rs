use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};


// basic logger for logging functions in server/client
pub struct Logger {
    pub service: &'static str
}

impl Logger {
    pub fn new(service: &'static str) -> Logger {
        Logger {service}
    }

    pub fn init(logger: &'static Logger) -> Result<(), SetLoggerError> {
        log::set_logger(logger).map(|()| log::set_max_level(LevelFilter::Info))
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{}: {} - {}", self.service, record.level(), record.args())
        }
    }

    fn flush(&self) {}
}
pub mod logger {

    use env_logger::Builder;
    use log::{debug, error, info, trace, warn, LevelFilter};

    use crate::cli_lib::OutputLevel;

    pub struct Logger {}

    #[allow(dead_code)]
    impl Logger {
        pub fn new(level: &OutputLevel) -> Self {
            let filter: LevelFilter = Logger::from_output_level(level);
            let mut builder = Builder::from_default_env();
            builder.filter_level(filter);
            builder.init();
            info!("Logger initialized with level: {:?}", filter);

            // Return a new instance.
            return Logger {};
        }

        pub fn test_all(&self) {
            info!("Information");
            error!("Error");
            warn!("Warning");
            debug!("Debug");
            trace!("Tracing");
        }

        // Create separate functions for all types of loggings
        pub fn info(&self, message: &str) {
            info!("{}", message);
        }

        pub fn error(&self, message: &str) {
            error!("{}", message);
        }

        pub fn warn(&self, message: &str) {
            warn!("{}", message);
        }

        pub fn debug(&self, message: &str) {
            debug!("{}", message);
        }

        pub fn trace(&self, message: &str) {
            trace!("{}", message);
        }

        pub fn create_instance(level: &OutputLevel) -> Logger {
            let logger: Logger = Logger::new(level);

            return logger;
        }

        fn from_output_level(level: &OutputLevel) -> LevelFilter {
            match level {
                OutputLevel::All => LevelFilter::Trace,
                OutputLevel::Default => LevelFilter::Error,
                OutputLevel::Info => LevelFilter::Info,
                OutputLevel::Warning => LevelFilter::Warn,
                OutputLevel::Error => LevelFilter::Error,
                OutputLevel::Debug => LevelFilter::Debug,
            }
        }
    }
}

pub mod logger {

    use env_logger::Builder;
    use log::{debug, error, info, trace, warn, LevelFilter};

    use crate::cli_lib::OutputLevel;

    pub struct Logger {}

    impl Logger {
        pub fn new(level: &OutputLevel) -> Self {
            let filter: LevelFilter = Logger::from_output_level(level);
            let mut builder = Builder::from_default_env();
            builder.filter_level(filter);
            builder.init();

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

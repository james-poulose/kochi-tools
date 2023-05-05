pub mod logger {
    use std::str::FromStr;

    use env_logger::Builder;
    use log::{debug, error, info, trace, warn, LevelFilter};

    pub struct Logger {}

    //impl LoggerX {}
    impl Logger {
        pub fn new(enum_string: &str) -> Self {
            let filter: LevelFilter = Logger::from_str(enum_string);
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

        fn from_str(input: &str) -> LevelFilter {
            match input {
                "Debug" => LevelFilter::Debug,
                "Error" => LevelFilter::Error,
                "Warn" => LevelFilter::Warn,
                "Default" => LevelFilter::Error,
                "Trace" => LevelFilter::Trace,
                "All" => LevelFilter::Trace,
                _ => LevelFilter::Error,
            }
        }
    }
}

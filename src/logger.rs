pub mod logger {
    use env_logger::Builder;

    pub fn init_logger() {
        let mut builder = Builder::from_default_env();
        builder.filter_level(log::LevelFilter::Info);
        builder.init();
    }
}

use crate::config::config::{LoggerConfig, CONFIG};
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

pub struct Logger;

impl Logger {
    pub fn init() -> () {
        let config: &LoggerConfig = &CONFIG.logger;

        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(config.level.as_str()));
        let formatting_layer = BunyanFormattingLayer::new(config.name.to_owned(), std::io::stdout);
        let subscriber = Registry::default()
            .with(env_filter)
            .with(JsonStorageLayer)
            .with(formatting_layer);

        set_global_default(subscriber).expect("Failed to set subscriber");
    }
}

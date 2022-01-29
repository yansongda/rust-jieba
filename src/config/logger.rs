use crate::Config;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

pub struct Logger;

impl Logger {
    pub fn init(config: &Config) -> () {
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(config.logger.level.to_owned()));
        let formatting_layer =
            BunyanFormattingLayer::new(config.app.name.to_owned(), std::io::stdout);
        let subscriber = Registry::default()
            .with(env_filter)
            .with(JsonStorageLayer)
            .with(formatting_layer);

        set_global_default(subscriber).expect("Failed to set subscriber");
    }
}

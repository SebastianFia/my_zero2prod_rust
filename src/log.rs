use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_appender::rolling::{RollingFileAppender, Rotation};

pub fn init_logger() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let file_appender = RollingFileAppender::new(
        Rotation::HOURLY,
        "./logs",
        "log",
    );
    let formatting_layer = BunyanFormattingLayer::new(
        "zero2prod".into(), 
        // std::io::stdout
        file_appender
    );
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");
}
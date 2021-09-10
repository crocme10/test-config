use clap::ArgMatches;
use snafu::{ResultExt, Snafu};
use tracing::{info, instrument};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

use super::settings::{Error as SettingsError, Settings as CSettings};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not get connection pool: {}", source))]
    Connection { source: Box<dyn std::error::Error> },

    #[snafu(display("Could not generate settings: {}", source))]
    Settings { source: SettingsError },

    #[snafu(display("Socket Addr Error {}", source))]
    SockAddr { source: std::io::Error },

    #[snafu(display("Addr Resolution Error {}", msg))]
    AddrResolution { msg: String },
}

#[allow(clippy::needless_lifetimes)]
pub async fn run<'a>(matches: &ArgMatches<'a>) -> Result<(), Error> {
    let settings = CSettings::new(matches).context(Settings)?;
    LogTracer::init().expect("Unable to setup log tracer!");

    let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();

    let file_appender = tracing_appender::rolling::daily(&settings.logging.path, "mimir.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let bunyan_formatting_layer = BunyanFormattingLayer::new(app_name, non_blocking);
    let subscriber = Registry::default()
        .with(EnvFilter::new("INFO"))
        .with(JsonStorageLayer)
        .with(bunyan_formatting_layer);
    tracing::subscriber::set_global_default(subscriber).expect("tracing subscriber global default");

    run_server(settings).await
}

#[instrument(skip(settings))]
pub async fn run_server(settings: CSettings) -> Result<(), Error> {
    info!("settings: {:?}", settings);
    Ok(())
}

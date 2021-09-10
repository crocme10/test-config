use clap::{App, Arg};
use snafu::{ResultExt, Snafu};

mod server;
mod settings;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Command Line Interface Error: {}", msg))]
    CLIError { msg: String },
    #[snafu(display("Server Error: {}", source))]
    ServerError {
        #[snafu(backtrace)]
        source: server::Error,
    },
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = App::new("Microservice for bragi")
        .version(VERSION)
        .author(clap::crate_authors!("\n"))
        .arg(
            Arg::with_name("config dir")
                .value_name("DIR")
                .short("c")
                .long("config-dir")
                .help("Config directory"),
        )
        .arg(
            Arg::with_name("run mode")
                .value_name("STRING")
                .short("m")
                .long("run_mode")
                .help("Run Mode: dev, prod, test"),
        )
        .get_matches();

    server::run(&matches).await.context(ServerError)
}

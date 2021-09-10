use clap::ArgMatches;
use config::{Config, File};
use serde::Deserialize;
use snafu::ResultExt;
use snafu::Snafu;
use std::env;
use std::path::Path;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Arg Match Error: {}", msg))]
    ArgMatch { msg: String },
    #[snafu(display("Arg Missing Error: {}", msg))]
    ArgMissing { msg: String },
    #[snafu(display("Env Var Missing Error: {} [{}]", msg, source))]
    EnvVarMissing { msg: String, source: env::VarError },
    #[snafu(display("Config Merge Error: {} [{}]", msg, source))]
    ConfigMerge {
        msg: String,
        source: config::ConfigError,
    },
    #[snafu(display("Config Extract Error: {} [{}]", msg, source))]
    ConfigExtract {
        msg: String,
        source: config::ConfigError,
    },
    #[snafu(display("Config Value Error: {} [{}]", msg, source))]
    ConfigValue {
        msg: String,
        source: std::num::TryFromIntError,
    },
    #[snafu(display("Config Value Error: {} [{}]", msg, source))]
    ConfigParse {
        msg: String,
        source: std::num::ParseIntError,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct ElasticsearchSettings {
    pub nb_replicas: u32,
    pub nb_shards: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ElasticsearchMappings {
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Elasticsearch {
    pub url: String,
    pub settings: ElasticsearchSettings,
    pub mappings: ElasticsearchMappings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logging {
    pub path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Service {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub mode: String,
    pub logging: Logging,
    pub elasticsearch: Elasticsearch,
    pub service: Service,
}

impl Settings {
    pub fn new<'a, T: Into<Option<&'a ArgMatches<'a>>>>(matches: T) -> Result<Self, Error> {
        let matches = matches.into().ok_or(Error::ArgMatch {
            msg: String::from("no matches"),
        })?;

        let config_dir = matches.value_of("config dir").ok_or(Error::ArgMissing {
            msg: String::from(
                "Missing config directory. You should use -c to specify a config directory",
            ),
        })?;

        let config_dir = Path::new(config_dir);

        let mut builder = Config::builder();

        let default_path = config_dir.join("default").with_extension("toml");

        // Start off by merging in the "default" configuration file
        builder = builder.add_source(File::from(default_path));

        // We use the RUN_MODE environment variable, and if it is not set, the command line
        // argument. If neither is present, we return an error.
        let run_mode = env::var("RUN_MODE").or_else(|_| {
            matches
                .value_of("run_mode")
                .ok_or_else(|| Error::ArgMissing {
                    msg: String::from("Missing run mode. You should either set the env var RUN_MODE, or use -m to specify a run mode"),
                })
                .map(ToOwned::to_owned)
        })?;

        let run_mode_path = config_dir.join(&run_mode).with_extension("toml");

        builder = builder.add_source(File::from(run_mode_path).required(true));

        let config = builder.build().context(ConfigMerge {
            msg: String::from("Cannot build configuration"),
        })?;

        config.try_into().context(ConfigMerge {
            msg: String::from("Cannot merge run_mode"),
        })
    }
}

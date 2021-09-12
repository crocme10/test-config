# Evaluating Config Crate

A sandbox to evaluate the functionality of the config crate. The crate config already has the
ability to have a layered configuration, that is a default configuration, which can be overriden by 
more specific configuration. This project looks into the following two issues:

- is it possible to split the configuration into several files, so that they can be reused separately
  in different contexts, while still beeing merged into the same result?
- is it possible to set arbitrary values in the configuration from the command line?

The result is yes to both questions, and the following code will prove it.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Getting Started

### Prerequisites

You need a rust development environment. See [Install
Rust](https://www.rust-lang.org/tools/install) for instructions.

### Installing

This is rust project, so the standard `cargo` incantation should do the trick:

```
git clone https://github.com/crocme10/test-config
cd test-config
cargo build --release
```

This will create a binary in `target/release/test-config`

## Running the tests

You must specify where the configuration files live. This is done with the '--config-dir' argument.

### Default and Run Mode

The first file that 'test-config' looks for is in '<config-dir>/default.toml'

Then 'test-config' searches for another config file which depends on your
'run-mode'. The run-mode is specified by the '--run-mode' argument, followed by
one of 'dev', 'test', or 'prod'. 'test-config' looks for a corresponding
'<config-dir>/<run-mode>.toml'. Any value previously set in
'<config-dir>/default.toml' will be overriden.

### Separare config files

The content of the config files depends a bit on what your needs are. In this example, we are
trying to populate the 'struct Settings' found in 'src/settings.rs'

'''rust
#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub mode: String,
    pub logging: Logging,
    pub elasticsearch: Elasticsearch,
    pub service: Service,
}
'''

and the 'struct Elasticsearch' is itself defined by

'''rust
#[derive(Debug, Clone, Deserialize)]
pub struct Elasticsearch {
    pub url: String,
    pub settings: serde_json::Value,
    pub mappings: serde_json::Value,
}
'''

To configure the settings and mappings in the 'struct Elasticsearch', we look
for json files which are by convention found in
'<config-dir>/elasticsearch/settings.json' and
'<config-dir>/elasticsearch/mappings.json'

At that point, if we run the application, it will simply create a log file and
store the configuration in there:

'''sh
./target/release/test-config -c config -m test
tail -f logs/test-config.<YYYY-MM-DD>
'''

It is important to notice that both the mapping file and setting file specify
the full path to the object that they are defining. So, the mappings, for
example, must start with '{ "elasticsearch": { "mappings": { ... } } }'

### Command line values

You can now override any configuration value by setting its value at the command line.
This is done by giving the path to the configuration key, followed by its value:

For example, if you want to set the number of replicas in your elastiscearch index, you can
give the following command line:

'''
./target/release/test-config -c config -m test -v elasticsearch.settings.index.number_of_replicas=1
'''

Currently, this works for boolean, integer, floating, and string values.

You can set any number of values with this mechanism:

'''
[...] -v KEY1=VALUE1 -v KEY2=VALUE2 ...
'''

## Built With

These are some of the crates used:

* [clap](https://docs.rs/juniper/0.14.2/juniper/) - Command line parsing
* [config](https://docs.rs/warp/0.2.3/warp/) - Configuration

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process
for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on
this repository](https://github.com/your/project/tags). 

## Authors

* **Matthieu Paindavoine** - *Initial work* - [crocme10](https://github.com/crocme10)

See also the list of [contributors](https://github.com/crocme10/test-config/contributors) who
participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

Thank you Billie Thompson for [Readme Template](https://github.com/PurpleBooth/a-good-readme-template#readme)

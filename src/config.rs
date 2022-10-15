use serde::Deserialize;

#[derive(Deserialize)]
pub struct WebConfig {
    pub addr: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub web: WebConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
}

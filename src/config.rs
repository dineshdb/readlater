use serde::Deserialize;

use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

#[derive(Deserialize)]
pub struct Config {
    pub pocket_consumer_key: String,
    pub pocket_access_token: String,
}

pub fn get_config() -> anyhow::Result<Config> {
    let config_dir = dirs::config_dir().expect("Could not find config directory");
    let base_dir = config_dir.join("readlater");
    std::fs::create_dir_all(&base_dir)?;
    let config_file = base_dir.join("config.toml");
    let config: Config = Figment::new()
        .merge(Toml::file(config_file))
        .merge(Env::prefixed("READLATER_"))
        .merge(Env::raw().only(&["POCKET_CONSUMER_KEY", "POCKET_ACCESS_TOKEN"]))
        .extract()?;

    Ok(config)
}

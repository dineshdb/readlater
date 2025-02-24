use directories::ProjectDirs;
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

pub fn get_dirs() -> ProjectDirs {
    ProjectDirs::from("", "", "readlater").expect("Could not find project directory")
}

pub fn get_config() -> anyhow::Result<Config> {
    let project_dirs = get_dirs();
    let config_dir = project_dirs.config_dir();
    std::fs::create_dir_all(project_dirs.config_local_dir())?;
    std::fs::create_dir_all(project_dirs.data_local_dir())?;

    let config_file = config_dir.join("config.toml");
    let config: Config = Figment::new()
        .merge(Toml::file(config_file))
        .merge(Env::prefixed("READLATER_"))
        .merge(Env::raw().only(&["POCKET_CONSUMER_KEY", "POCKET_ACCESS_TOKEN"]))
        .extract()?;

    Ok(config)
}

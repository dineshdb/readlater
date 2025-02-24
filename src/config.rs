use directories::ProjectDirs;
use serde::Deserialize;
use std::path::PathBuf;

pub const DATABASE_PATH: &str = "readlater.sqlite";
pub const POCKET_CONSUMER_KEY: &str = "113896-1812a82dd99b90ac1835fd5";
pub const POCKET_REDIRECT_URI: &str = "https://localhost:8080/auth/pocket/callback";

#[derive(Deserialize)]
pub struct Config {
    pub pocket_consumer_key: String,
    pub database_dir: PathBuf,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let project_dirs =
            ProjectDirs::from("", "", "readlater").expect("Could not find project directory");
        let database_dir = project_dirs.data_local_dir().join(DATABASE_PATH);
        std::fs::create_dir_all(project_dirs.config_local_dir())?;
        std::fs::create_dir_all(project_dirs.data_local_dir())?;
        std::fs::File::open(database_dir.join("readlater.sqlite"))?;
        Ok(Self {
            pocket_consumer_key: POCKET_CONSUMER_KEY.to_string(),
            database_dir,
        })
    }
}

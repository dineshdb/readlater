use std::{
    env,
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Manifest {
    pub name: String,
    pub description: String,
    pub path: PathBuf,
    #[serde(rename = "type")]
    pub io_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_extensions: Option<Vec<String>>,
}

pub fn install_linux(manifest: &Manifest) -> std::io::Result<()> {
    let manifest_json = serde_json::to_string_pretty(manifest).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Serialization failed: {}", e))
    })?;
    let home_dir = env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let path = PathBuf::from(format!(
        "{}/.mozilla/native-messaging-hosts/{}.json",
        home_dir, manifest.name
    ));
    write_file(&path, &manifest_json)
}

fn write_file(filename: &PathBuf, contents: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(contents.as_bytes())
}

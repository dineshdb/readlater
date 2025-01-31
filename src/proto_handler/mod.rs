use std::{env, path::PathBuf};

pub fn register_url_handler() {
    let cli = env::current_exe().unwrap();
    let cli = cli.to_str().to_owned().unwrap();
    #[cfg(target_os = "linux")]
    register_linux(String::from(cli), "readlater".to_string());

    #[cfg(not(any(target_os = "linux")))]
    println!("Unsupported operating system");
}

#[cfg(target_os = "linux")]
fn register_linux(cli: String, scheme: String) {
    use std::{fs::File, io::Write, process::Command};

    let desktop_entry = format!(
        r#"[Desktop Entry]
Type=Application
Name={scheme} URL Handler
Exec={cli} handle --url %u
StartupNotify=false
MimeType=x-scheme-handler/{scheme};"#
    );

    let home_dir = env::var("HOME").unwrap();
    let apps_dir = PathBuf::from(&home_dir).join(".local/share/applications");
    std::fs::create_dir_all(&apps_dir).unwrap();

    let desktop_file_path = apps_dir.join(format!("{scheme}-url-handler.desktop"));
    let mut file = File::create(desktop_file_path).unwrap();
    file.write_all(desktop_entry.as_bytes()).unwrap();

    Command::new("xdg-mime")
        .args([
            "default",
            &format!("{scheme}-url-handler.desktop"),
            &format!("x-scheme-handler/{scheme}"),
        ])
        .output()
        .expect("Failed to register MIME type");

    println!("{scheme} handler registered for Linux");
}

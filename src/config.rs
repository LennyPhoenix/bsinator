use std::path::PathBuf;
use std::{fs, io};

const CONFIG_FILE: &str = "config.yml";

pub fn get_config_path() -> Option<PathBuf> {
    let mut home = home::home_dir()?;
    home.push(".config/bsinator");
    if home.exists() {
        Some(home)
    } else {
        None
    }
}

pub fn traverse_config_directory(config_path: PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut modules = Vec::new();
    for entry in fs::read_dir(config_path)? {
        if let Ok(entry) = entry {
            let mut config = entry.path();
            config.push(CONFIG_FILE);
            if config.exists() {
                modules.push(config);
            } else {
                let submodules = traverse_config_directory(entry.path())?;
                modules.extend(submodules);
            }
        }
    }
    Ok(modules)
}

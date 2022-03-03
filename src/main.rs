use std::path::PathBuf;
use std::{fs, io};

const CONFIG_FILE: &str = "config.yml";

fn get_config_path() -> Option<PathBuf> {
    let mut home = home::home_dir()?;
    home.push(".config/bsinator");
    if home.exists() {
        Some(home)
    } else {
        None
    }
}

fn traverse_config_directory(config_path: PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut modules = Vec::new();
    std::thread::sleep(std::time::Duration::from_secs(5));
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

fn main() {
    match get_config_path() {
        Some(path) => {
            match traverse_config_directory(path) {
                Ok(modules) => {
                    for module in modules {
                        println!("{}", module.display());
                    }
                },
                Err(err) => {
                    println!("Failed to traverse config: {}", err);
                }
            }
        }
        None => println!("No config path found."),
    }
}

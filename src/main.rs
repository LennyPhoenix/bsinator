mod config;
mod modules;

fn main() {
    match config::get_config_path() {
        Some(path) => match config::traverse_config_directory(path) {
            Ok(modules) => {
                for module in modules {
                    println!("{}", module.display());
                }
            }
            Err(err) => {
                println!("Failed to traverse config: {}", err);
                println!("Please do not modify the config directory while bsinator is running.");
            }
        },
        None => println!("No config path found."),
    }
}

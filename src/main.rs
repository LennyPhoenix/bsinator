mod config;
mod modules;

fn main() {
    match config::get_config_path() {
        Some(path) => match modules::parse_modules(path) {
            Ok(modules) => {
                for module in modules {
                    println!("{}", module.name);
                    if let Some(dependencies) = module.dependencies {
                        for dep in dependencies {
                            println!("\t{:?}", dep.packages);
                        }
                    }
                }
            }
            Err(err) => {
                println!("Failed to parse config: {}", err);
                println!("Please do not modify the config directory while bsinator is running.");
            }
        },
        None => println!("No config path found."),
    }
}

use crate::api::types;
use configr::Config;
use directories::ProjectDirs;
use std::fs;

pub fn load_config() -> types::Secret {
    let location = ProjectDirs::from("io", "TreuKS", "treuksbot").unwrap();

    let file = match fs::read_to_string(format!(
        "{}/config.toml",
        &location.config_dir().to_str().unwrap()
    )) {
        Ok(okay) => okay,
        Err(_err) => {
            // If the config file wasn't detected
            eprintln!("Couldn't find the config file.");
            println!("Recreating the config file.");
            match fs::create_dir_all(location.config_dir()) {
                Ok(()) => {
                    let config_file = fs::File::create(format!(
                        "{}/config.toml",
                        location.config_dir().to_str().unwrap()
                    ))
                    .unwrap();

                    println!("OK: File has been created");

                    types::Secret::populate_template(config_file)
                        .expect("Couldn't fill in the file");
                    println!("OK: Configuration file has been populated with a template");
                    println!(
                        "You need to go and edit the {}/config.toml file with correct data.",
                        &location.config_dir().to_str().unwrap()
                    );
                    std::process::exit(0);
                }
                Err(er) => {
                    panic!("Couldn't create a directory, {}", er)
                }
            }
        }
    };

    return match toml::from_str(&file) {
        // If the config file is incorrect
        Ok(ok) => ok,
        Err(_er) => {
            eprintln!("The TOML file is incorrect. Please fix it.");
            eprintln!("You can also delete the file and it will be recreated.");
            std::process::exit(1);
        }
    };
}

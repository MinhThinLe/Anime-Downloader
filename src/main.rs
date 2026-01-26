mod config_parser;
mod download;
mod update_config;

use std::process::exit;

use config_parser::*;
use download::download;

fn main() {
    let config = parse_config();
    let mut config = match config {
        Ok(config) => config,
        Err(ParseConfigError::FileNotFound) => {
            make_config();
            Config::default()
        }
        Err(ParseConfigError::InvalidTOML) => {
            println!("Invalid Config");
            exit(1)
        }
    };
    loop {
        match download(&mut config) {
            Ok(_) => (),
            Err(error) => {
                println!("{error}, exiting now");
                exit(1)
            }
        };
        config.write_to_disk();

        println!(
            "Nothing of interest, going to sleep for {} seconds",
            config.get_sleep_duration().as_secs()
        );
        std::thread::sleep(config.get_sleep_duration());
    }
}

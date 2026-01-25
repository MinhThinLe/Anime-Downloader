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
    download(&mut config);
    config.write_to_disk();
}

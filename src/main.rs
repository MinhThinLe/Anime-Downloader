mod config_parser;
mod download;
mod update_state;
mod paths;

use std::process::exit;

use config_parser::*;

fn main() {
    let mut app = match App::new_from_config() {
        Ok(app) => app,
        Err(ParseConfigError::InvalidTOML) => {
            println!("Invalid configuration file");
            exit(1);
        }
    };
    app.main_loop();
}

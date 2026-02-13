mod config_parser;
mod download;
mod paths;
mod rename;
mod update_state;

use std::process::exit;

use config_parser::*;

fn main() {
    let Ok(mut app) = App::new_from_config() else {
        println!("Invalid configuration file");
        exit(1);
    };
    loop {
        app.main_loop();
    }
}

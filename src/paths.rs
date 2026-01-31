use std::process::exit;
use std::fs::{self, File, exists };
use std::path::PathBuf;

const CONFIG_PATH: &str = ".config/anime-dowloader/";
const CONFIG_FILE: &str = "watchlist.toml";

fn make_config_path() {
    let config_path = get_config_path();
    match fs::create_dir_all(&config_path) {
        Ok(_) => (),
        Err(error) => {
            println!("Unable to mkdir {:?} due to error {}, exiting now", config_path, error);
            exit(1);
        }
    }
}

fn make_config_file() {
    let config_file = get_config_file_path();
    match File::create(&config_file) {
        Ok(_) => (),
        Err(error) => {
            println!("Unable to touch {:?} due to error {}, exiting now", config_file, error);
            exit(1);
        }
    }
}

pub fn make_config() {
    let Ok(config_path_exists) = exists(get_config_path()) else {
        println!("Something has really gone wrong");
        exit(1);
    };
    if !config_path_exists {
        make_config_path();
    }
    let Ok(config_file_exists) = exists(get_config_file_path()) else {
        println!("Can't read the system's file structure, maybe you've got a permission issue?");
        exit(1);
    };
    if !config_file_exists {
        make_config_file();
    }
}

pub fn get_config_file_path() -> PathBuf {
    let mut config_path = get_config_path();
    config_path.push(CONFIG_FILE);
    return config_path;
}

pub fn get_config_path() -> PathBuf {
    let mut home_path = get_home_path();
    home_path.push(CONFIG_PATH);
    return home_path;
}

pub fn get_home_path() -> PathBuf {
    match std::env::home_dir() {
        Some(dir) => return dir,
        None => {
            println!("Couldn't get the user's home directory, exiting");
            exit(1);
        }
    };
}

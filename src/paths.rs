use std::fs::{self, File, exists};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::exit;

const CONFIG_PATH: &str = ".config/anid/";
const CONFIG_FILE: &str = "watchlist.toml";
const STATE_PATH: &str = ".local/state/anid/";
const STATE_FILE: &str = "anime-downloader.state";
const CONFIG_FILE_CONTENT: &str = include_str!("../examples/watchlist.toml");

const ERROR_UNREADABLE_FILESYSTEM: &str =
    "Unable to read the system's file structure, maybe you've got a permission issue?";

fn make_config_path() {
    let config_path = get_config_path();
    match fs::create_dir_all(&config_path) {
        Ok(_) => (),
        Err(error) => {
            println!(
                "Unable to mkdir {:?} due to {}, exiting now",
                config_path, error
            );
            exit(1);
        }
    }
}

fn make_config_file() {
    let config_file = get_config_file_path();
    let mut config_file = match File::create(&config_file) {
        Ok(config_file) => config_file,
        Err(error) => {
            println!(
                "Unable to touch {:?} due to {}, exiting now",
                config_file, error
            );
            exit(1);
        }
    };

    if let Err(error) = config_file.write_all(CONFIG_FILE_CONTENT.as_bytes()) {
        println!("Unable to create config file due to {error}");
    }
}

fn make_state_path() {
    let state_path = get_state_path();
    match fs::create_dir_all(&state_path) {
        Ok(_) => (),
        Err(error) => {
            println!(
                "Unable to mkdir {:?} due to {}, exiting now",
                state_path, error
            );
            exit(1);
        }
    }
}

fn make_state_file() {
    let state_file_path = get_state_file_path();
    match File::create(&state_file_path) {
        Ok(_) => (),
        Err(error) => {
            println!(
                "Unable to touch {:?} due to {}, exiting now",
                state_file_path, error
            );
            exit(1);
        }
    }
}

pub fn make_config() {
    let Ok(config_path_exists) = exists(get_config_path()) else {
        println!("{}", ERROR_UNREADABLE_FILESYSTEM);
        exit(1);
    };
    if !config_path_exists {
        make_config_path();
    }
    let Ok(config_file_exists) = exists(get_config_file_path()) else {
        println!("{}", ERROR_UNREADABLE_FILESYSTEM);
        exit(1);
    };
    if !config_file_exists {
        make_config_file();
    }
}

pub fn make_state() {
    let Ok(state_path_exists) = exists(get_state_path()) else {
        println!("{}", ERROR_UNREADABLE_FILESYSTEM);
        exit(1);
    };

    if !state_path_exists {
        make_state_path();
    }

    let Ok(state_file_exists) = exists(get_state_file_path()) else {
        println!("{}", ERROR_UNREADABLE_FILESYSTEM);
        exit(1);
    };

    if !state_file_exists {
        make_state_file();
    }
}

pub fn make_directory(directory: &Path) {
    match fs::create_dir_all(directory) {
        Ok(_) => (),
        Err(error) => {
            println!(
                "Unable to create directory {:?} due to {}, exiting now",
                directory, error
            );
            exit(1);
        }
    }
}

pub fn get_config_file_path() -> PathBuf {
    let mut config_path = get_config_path();
    config_path.push(CONFIG_FILE);
    config_path
}

pub fn get_config_path() -> PathBuf {
    let mut home_path = get_home_path();
    home_path.push(CONFIG_PATH);
    home_path
}

pub fn get_state_file_path() -> PathBuf {
    let mut state_path = get_state_path();
    state_path.push(STATE_FILE);
    state_path
}

pub fn get_state_path() -> PathBuf {
    let mut home_path = get_home_path();
    home_path.push(STATE_PATH);
    home_path
}

pub fn get_home_path() -> PathBuf {
    match std::env::home_dir() {
        Some(dir) => dir,
        None => {
            println!("Couldn't get the user's home directory, exiting");
            exit(1);
        }
    }
}

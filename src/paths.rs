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

pub fn make_config() {
    let Ok(config_path_exists) = exists(get_config_path()) else {
        println!("{}", ERROR_UNREADABLE_FILESYSTEM);
        exit(1);
    };
    if !config_path_exists {
        make_directory(&get_config_path());
    }
    let Ok(config_file_exists) = exists(get_config_file_path()) else {
        println!("{}", ERROR_UNREADABLE_FILESYSTEM);
        exit(1);
    };
    if !config_file_exists {
        let mut config_file = make_file(&get_config_file_path());

        if let Err(error) = config_file.write_all(CONFIG_FILE_CONTENT.as_bytes()) {
            println!("Unable to write to config file due to error {error}");
        }
    }
}

pub fn make_state() {
    let Ok(state_path_exists) = exists(get_state_path()) else {
        println!("{}", ERROR_UNREADABLE_FILESYSTEM);
        exit(1);
    };

    if !state_path_exists {
        make_directory(&get_state_path());
    }

    let Ok(state_file_exists) = exists(get_state_file_path()) else {
        println!("{}", ERROR_UNREADABLE_FILESYSTEM);
        exit(1);
    };

    if !state_file_exists {
        make_file(&get_state_file_path());
    }
}

pub fn make_directory(directory: &Path) {
    if let Err(error) = fs::create_dir_all(directory) {
        println!(
            "Unable to create directory {:?} due to {}, exiting now",
            directory, error
        );
        exit(1);
    }
}

pub fn make_file(file_path: &Path) -> File {
    match File::create(&file_path) {
        Ok(file) => file,
        Err(error) => {
            println!(
                "Unable to touch {:?} due to {}, exiting now",
                file_path, error
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
    let Some(dir) = std::env::home_dir() else {
        println!("Couldn't get the user's home directory, exiting");
        exit(1);
    };
    dir
}

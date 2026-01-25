use std::fs::{self, File};
use std::path::{PathBuf, Path};
use std::process::exit;

use toml::{Table, Value};

const CONFIG_PATH: &str = ".config/anime-dowloader/";
pub const CONFIG_FILE: &str = "watchlist.toml";

#[derive(Debug, Default)]
pub struct Config {
    pub watch_list: Vec<AnimeEntry>,
}

#[derive(Debug)]
pub struct AnimeEntry {
    id :Box<str>,
    name: Box<str>,
    current_episode: u16,
    target_dir: PathBuf,
    entry_number: Option<u8>,
}

#[derive(Debug)]
pub enum ParseConfigError {
    FileNotFound,
    InvalidTOML,
}

fn get_entry_number(table: &Value) -> Option<u8> {
    Some(table.get("select")?.as_integer()? as u8)
}

impl AnimeEntry {
    fn from_table(id: String, table: Value) -> Option<Self> {
        let name = table.get("name")?.as_str()?.into();
        let current_episode = table.get("current_episode")?.as_integer()? as u16;
        let target_dir = table.get("directory")?.as_str()?.into();
        let id = id.into();

        let entry_number = get_entry_number(&table);

        Some(Self {
            id,
            name,
            entry_number,
            target_dir,
            current_episode,
        })
    }

    pub fn get_download_arguments(&self) -> Vec<String> {
        let mut args: Vec<String> = vec![
            "-d".to_string(),
            "-e".to_string(),
            self.current_episode.to_string(),
            self.name.to_string(),
        ];
        if let Some(select) = self.entry_number {
            args.extend_from_slice(&[
                "-S".to_string(),
                select.to_string(),
            ]);
        }

        args
    }

    pub fn get_target_directory(&self) -> &Path {
        self.target_dir.as_path()
    }

    pub fn next_episode(&mut self) {
        self.current_episode += 1;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
    
    pub fn get_entry_number(&self) -> Option<u8> {
        self.entry_number
    }

    pub fn get_current_episode(&self) -> u16 {
        self.current_episode
    }
}

pub fn parse_config() -> Result<Config, ParseConfigError> {
    let mut config_file = get_config_path();
    config_file.push(CONFIG_FILE);
    let config_file = fs::read_to_string(config_file).or(Err(ParseConfigError::FileNotFound))?;
    let parsed = config_file
        .parse::<Table>()
        .or(Err(ParseConfigError::InvalidTOML))?;

    let mut config = Config::default();

    for item in parsed {
        if (item.0) == "config" {
            // TODO add configuration options here
            continue;
        }
        if let Some(entry) = AnimeEntry::from_table(item.0, item.1) {
            config.watch_list.push(entry);
        }
    }

    Ok(config)
}

pub fn make_config() {
    let mut config_path = get_config_path();
    let error = fs::create_dir_all(&config_path);
    match error {
        Ok(_) => (),
        Err(error) => {
            println!("Encountered error {error} while making config directory");
            exit(1);
        }
    }
    config_path.push(CONFIG_FILE);
    let error = File::create(config_path);
    match error {
        Ok(_) => (),
        Err(error) => {
            println!("Encountered error {error} while making config file");
            exit(1);
        }
    }
}

pub fn get_config_path() -> PathBuf {
    let mut home = match std::env::home_dir() {
        Some(dir) => dir,
        None => {
            println!("Couldn't get the user's home directory, exiting");
            exit(1);
        }
    };
    home.push(CONFIG_PATH);
    home
}

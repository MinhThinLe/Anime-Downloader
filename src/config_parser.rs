use crate::paths::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use toml::map::Map;
use toml::{Table, Value};

const DEFAULT_SLEEP_SECONDS: u64 = 24 * 60 * 60;
const DEFAULT_DOWNLOAD_PATH: &str = "/tmp/anid/";

#[derive(Debug, Default)]
pub struct App {
    pub watch_list: Vec<AnimeEntry>,
    settings: Settings,
}

#[derive(Debug)]
pub struct Settings {
    sleep_duration: Duration,
    dowload_location: PathBuf,
}

#[derive(Debug)]
pub struct AnimeEntry {
    id: Box<str>,
    name: Box<str>,
    current_episode: u16,
    target_dir: PathBuf,
    entry_number: Option<u8>,
    rename_pattern: Option<String>,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidTOML,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            sleep_duration: Duration::from_secs(DEFAULT_SLEEP_SECONDS),
            dowload_location: PathBuf::from(DEFAULT_DOWNLOAD_PATH),
        }
    }
}

impl App {
    pub fn get_sleep_duration(&self) -> Duration {
        self.settings.sleep_duration
    }

    pub fn get_temp_path(&self) -> PathBuf {
        self.settings.dowload_location.clone()
    }

    fn resume_from_state(&mut self) {
        let Ok(parsed) = read_state() else {
            return;
        };

        for (table_name, value) in parsed {
            let Some(entry) = self.get_entry_mut(&table_name) else {
                continue;
            };
            entry.current_episode = value
                .get("current_episode")
                .expect("INVALID STATEFILE")
                .as_integer()
                .expect("INVALID STATEFILE") as u16;
        }
    }

    fn get_entry_mut(&mut self, entry_id: &str) -> Option<&mut AnimeEntry> {
        self.watch_list
            .iter_mut()
            .find(|entry| entry.get_id() == entry_id)
    }

    pub fn new_from_config() -> Result<App, ParseError> {
        let parsed = read_config()?;
        let mut app = App::default();

        for (table_name, value) in parsed {
            if (table_name) == "config" {
                app.settings = parse_settings(&value);
                continue;
            }
            if let Some(entry) = AnimeEntry::from_config_table(table_name, value) {
                app.watch_list.push(entry);
            }
        }

        app.resume_from_state();
        Ok(app)
    }
}

impl AnimeEntry {
    fn from_config_table(id: String, table: Value) -> Option<Self> {
        fn get_entry_number(table: &Value) -> Option<u8> {
            Some(table.get("select")?.as_integer()? as u8)
        }

        fn get_rename_pattern(table: &Value) -> Option<String> {
            let Some(pattern) = table.get("rename") else {
                return None;
            };

            let Some(pattern) = pattern.as_str() else {
                println!("Warning: rename must be a string");
                return None;
            };

            if pattern.is_empty() {
                return None;
            }

            Some(pattern.to_string())
        }

        let name = table.get("name")?.as_str()?.into();
        let target_dir = table.get("directory")?.as_str()?.into();
        let id = id.into();

        let entry_number = get_entry_number(&table);
        let rename_pattern = get_rename_pattern(&table);

        Some(Self {
            id,
            name,
            entry_number,
            target_dir,
            rename_pattern,
            current_episode: 1,
        })
    }

    pub fn get_download_arguments(&self) -> Vec<String> {
        let mut args: Vec<String> = vec![
            "-d".to_string(),
            "-e".to_string(),
            self.current_episode.to_string(),
            self.name.to_string(),
        ];
        if let Some(select) = self.get_entry_number() {
            args.extend_from_slice(&["-S".to_string(), select.to_string()]);
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

    pub fn get_rename_pattern(&self) -> Option<&str> {
        self.rename_pattern.as_deref()
    }
}

fn parse_settings(table: &Value) -> Settings {
    fn get_sleep_time(table: &Value) -> u64 {
        let Some(sleep_time) = table.get("sleep_secs") else {
            return DEFAULT_SLEEP_SECONDS;
        };
        let Some(sleep_time) = sleep_time.as_integer() else {
            return DEFAULT_SLEEP_SECONDS;
        };
        sleep_time as u64
    }

    fn get_temp_path(table: &Value) -> PathBuf {
        let Some(path) = table.get("temp_path") else {
            return DEFAULT_DOWNLOAD_PATH.into();
        };
        let Some(path) = path.as_str() else {
            println!(
                "Warning: {:?} isn't a valid path, defaulting to {}",
                path, DEFAULT_DOWNLOAD_PATH
            );
            return DEFAULT_DOWNLOAD_PATH.into();
        };

        return path.into();
    }

    Settings {
        sleep_duration: Duration::from_secs(get_sleep_time(table)),
        dowload_location: get_temp_path(table),
    }
}

fn read_config() -> Result<Map<String, Value>, ParseError> {
    let config_file = get_config_file_path();
    let Ok(config_file) = fs::read_to_string(config_file) else {
        make_config();
        return read_config();
    };
    config_file
        .parse::<Table>()
        .or(Err(ParseError::InvalidTOML))
}

fn read_state() -> Result<Map<String, Value>, ParseError> {
    let state_file = get_state_file_path();
    let Ok(state_file) = fs::read_to_string(state_file) else {
        make_state();
        return read_state();
    };

    state_file.parse::<Table>().or(Err(ParseError::InvalidTOML))
}

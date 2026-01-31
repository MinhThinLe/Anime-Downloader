use crate::paths::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use toml::{Table, Value};

const DEFAULT_SLEEP_SECONDS: u64 = 24 * 60 * 60;

#[derive(Debug, Default)]
pub struct App {
    pub watch_list: Vec<AnimeEntry>,
    settings: Settings,
}

#[derive(Debug)]
pub struct Settings {
    sleep_duration: Duration,
}

#[derive(Debug)]
pub struct AnimeEntry {
    id: Box<str>,
    name: Box<str>,
    current_episode: u16,
    target_dir: PathBuf,
    entry_number: Option<u8>,
}

#[derive(Debug)]
pub enum ParseConfigError {
    InvalidTOML,
}

fn get_entry_number(table: &Value) -> Option<u8> {
    Some(table.get("select")?.as_integer()? as u8)
}

fn get_sleep_time(table: &Value) -> u64 {
    let Some(sleep_time) = table.get("sleep_secs") else {
        return DEFAULT_SLEEP_SECONDS;
    };
    let Some(sleep_time) = sleep_time.as_integer() else {
        return DEFAULT_SLEEP_SECONDS;
    };
    sleep_time as u64
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            sleep_duration: Duration::from_secs(DEFAULT_SLEEP_SECONDS),
        }
    }
}

impl App {
    pub fn get_sleep_duration(&self) -> Duration {
        self.settings.sleep_duration
    }

    pub fn new_from_config() -> Result<App, ParseConfigError> {
        let config_file = get_config_file_path();
        let Ok(config_file) = fs::read_to_string(config_file) else {
            make_config();
            // If you fail at first, simply try again
            return App::new_from_config();
        };
        let parsed = config_file
            .parse::<Table>()
            .or(Err(ParseConfigError::InvalidTOML))?;

        let mut app = App::default();

        for item in parsed {
            if (item.0) == "config" {
                app.settings = parse_settings(&item.1);
                continue;
            }
            if let Some(entry) = AnimeEntry::from_table(item.0, item.1) {
                app.watch_list.push(entry);
            }
        }

        Ok(app)
    }
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
}

fn parse_settings(table: &Value) -> Settings {
    Settings {
        sleep_duration: Duration::from_secs(get_sleep_time(table)),
    }
}

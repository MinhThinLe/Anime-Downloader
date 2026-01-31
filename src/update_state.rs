use std::fmt::Display;
use std::fs::write;

use crate::paths::*;
use crate::{AnimeEntry, App};

impl App {
    pub fn write_to_disk(&self) {
        let config_file = get_config_file_path();

        let error = write(config_file, self.to_string());
        match error {
            Ok(_) => (),
            Err(error) => println!("Couldn't write file due to {}", error),
        }
    }
}

impl Display for AnimeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut serialized = String::new();
        serialized.push_str(&format!("[{}]\n", self.get_id()));
        serialized.push_str(&format!("name = \"{}\"\n", self.get_name()));
        serialized.push_str(&format!(
            "directory = \"{}\"\n",
            self.get_target_directory().to_string_lossy()
        ));
        if let Some(entry) = self.get_entry_number() {
            serialized.push_str(&format!("select = {}\n", entry));
        }
        serialized.push_str(&format!(
            "current_episode = {}\n",
            self.get_current_episode()
        ));

        serialized.push('\n');
        write!(f, "{}", serialized)
    }
}

impl Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut serialized = String::new();

        serialized.push_str("[config]\n");
        serialized.push_str(&format!(
            "sleep_secs = {}\n",
            self.get_sleep_duration().as_secs()
        ));
        serialized.push('\n');

        for entry in self.watch_list.iter() {
            serialized.push_str(&entry.to_string());
        }

        write!(f, "{}", serialized)
    }
}

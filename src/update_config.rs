use std::fs::write;

use crate::{ Config, AnimeEntry };
use crate::get_config_path;
use crate::CONFIG_FILE;

impl Config {
    pub fn write_to_disk(&self) {
        let mut config_path = get_config_path();
        config_path.push(CONFIG_FILE);

        let error = write(config_path, self.to_string());
        match error {
            Ok(_) => (),
            Err(error) => println!("Couldn't write file due to {}", error),
        }
    }
}

impl ToString for AnimeEntry {
    fn to_string(&self) -> String {
        let mut serialized = String::new();
        serialized.push_str(&format!("[{}]\n", self.get_id()));
        serialized.push_str(&format!("name = \"{}\"\n", self.get_name()));
        serialized.push_str(&format!("directory = \"{}\"\n", self.get_target_directory().to_string_lossy()));
        if let Some(entry) = self.get_entry_number() {
            serialized.push_str(&format!("select = {}\n", entry));
        }
        serialized.push_str(&format!("current_episode = {}\n", self.get_current_episode()));

        serialized.push('\n');
        serialized
    }
}

impl ToString for Config {
    fn to_string(&self) -> String {
        let mut serialized = String::new();

        for entry in self.watch_list.iter() {
            serialized.push_str(&entry.to_string());
        }

        serialized
    }
}

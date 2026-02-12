use std::fs::{ReadDir, copy, exists, read_dir, remove_file, rename};
use std::path::Path;
use std::process::{Command, exit};

use crate::paths::make_directory;
use crate::{AnimeEntry, App};

const SUCCESS: i32 = 0;
const COMMAND_NOT_FOUND: i32 = 127;
const FAILURE: i32 = 1;

impl App {
    fn download(&mut self) -> Result<(), String> {
        let temp_path = self.get_temp_path();

        if !exists(&temp_path).expect("Can't read the filesystem") {
            make_directory(&self.get_temp_path());
        }

        for entry in self.watch_list.iter_mut() {
            let downloaded = entry.download(&temp_path)?;
            if !downloaded {
                continue;
            }
            if !exists(entry.get_target_directory()).expect("Can't read the filesystem") {
                make_directory(entry.get_target_directory());
            }
            if entry.get_rename_pattern().is_some() {
                rename_downloaded_files(&temp_path, entry);
            }
            move_downloaded_files(&temp_path, entry);

            entry.next_episode();
        }
        Ok(())
    }

    pub fn main_loop(&mut self) {
        if let Err(error) = self.download() {
            println!("{error}, exiting now");
            exit(1);
        }
        self.write_state_file();
        println!(
            "All tasks finished, going to sleep for {} seconds",
            self.get_sleep_duration().as_secs()
        );
        self.sleep();
    }

    fn sleep(&self) {
        std::thread::sleep(self.get_sleep_duration());
    }
}

impl AnimeEntry {
    fn download(&mut self, download_path: &Path) -> Result<bool, String> {
        println!(
            "Downloading episode {} of {}",
            self.get_current_episode(),
            self.get_name()
        );
        let args = self.get_download_arguments();
        let downloader = Command::new("ani-cli")
            .current_dir(download_path)
            .args(args)
            .status();

        let code = match downloader {
            Ok(status_code) => status_code.code(),
            Err(error) => return Err(error.to_string()),
        };
        let Some(code) = code else {
            return Err("An unknown error occurred".to_string());
        };

        match code {
            SUCCESS => return Ok(true),
            FAILURE => (),
            COMMAND_NOT_FOUND => println!("ani-cli executable not found, maybe try installing it?"),
            code => println!("Unknown return code {code}"),
        };

        Ok(false)
    }
}

fn list_files(path: &Path) -> ReadDir {
    let Ok(files) = read_dir(path) else {
        println!("Unable to read temporary directory {:?}, exiting now", path);
        exit(1);
    };
    files
}

fn rename_downloaded_files(temp_path: &Path, downloaded_entry: &AnimeEntry) {
    let files = list_files(temp_path);
    for file in files {
        let file = file.expect("Huh?");
        let path = file.path();
        if path.is_dir() {
            continue;
        }
        let extension = path
            .extension()
            .expect("Item should have a file extension")
            .to_string_lossy();
        let new_name = format!("{}.{}", downloaded_entry.get_new_name(), extension);
        let mut new_path = path.parent().expect("Shouldn't be at root").to_path_buf();
        new_path.push(new_name);
        rename(&path, &new_path).expect("Unable to rename file(s)");
    }
}

fn move_downloaded_files(temp_path: &Path, downloaded_entry: &AnimeEntry) {
    let files = list_files(temp_path);
    for file in files {
        let file = file.expect("Huh?");
        let old_path = file.path();
        if old_path.is_dir() {
            continue;
        }
        let file_name = old_path.file_name().expect("Can't read filename");
        let mut new_path = downloaded_entry.get_target_directory().to_path_buf();
        new_path.push(file_name);

        if let Err(_) = rename(&old_path, &new_path) {
            copy(&old_path, &new_path).expect("Unable to copy file");
            remove_file(&old_path).expect("Unable to remove file");
            continue;
        }
    }
}

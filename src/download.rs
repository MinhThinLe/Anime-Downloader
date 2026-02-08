use crate::{AnimeEntry, App, paths::make_download_directory};
use std::{
    fs::exists,
    process::{Command, exit},
};

const SUCCESS: i32 = 0;
const COMMAND_NOT_FOUND: i32 = 127;
const FAILURE: i32 = 1;

impl App {
    fn download(&mut self) -> Result<(), String> {
        for entry in self.watch_list.iter_mut() {
            entry.download()?;
        }
        Ok(())
    }

    pub fn main_loop(&mut self) {
        if let Err(error) =  self.download() {
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
    fn download(&mut self) -> Result<(), String> {
        if !exists(self.get_target_directory()).expect("Can't read the filesystem") {
            make_download_directory(self.get_target_directory());
        }

        println!("Downloading {}", self.get_name());
        let args = self.get_download_arguments();
        let downloader = Command::new("ani-cli")
            .current_dir(self.get_target_directory())
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
            SUCCESS => self.next_episode(),
            COMMAND_NOT_FOUND => println!("ani-cli executable not found, maybe try installing it?"),
            FAILURE => (),
            code => println!("Unknown return code {code}"),
        };

        Ok(())
    }
}

use crate::{AnimeEntry, Config};
use std::process::Command;

const SUCCESS: i32 = 0;
const COMMAND_NOT_FOUND: i32 = 127;
const FAILURE: i32 = 1;

pub fn download(config: &mut Config) -> Result<(), String> {
    for entry in config.watch_list.iter_mut() {
        download_entry(entry)?;
    }
    Ok(())
}

fn download_entry(entry: &mut AnimeEntry) -> Result<(), String> {
    let args = entry.get_download_arguments();
    let downloader = Command::new("ani-cli")
        .current_dir(entry.get_target_directory())
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
        SUCCESS => entry.next_episode(),
        COMMAND_NOT_FOUND => println!("ani-cli executable not found, maybe try installing it?"),
        FAILURE => (),
        code => println!("Unknown return code {code}"),
    };

    Ok(())
}

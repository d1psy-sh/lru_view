/// here we define the open commands
/// first we use xdg-open to open the file
/// in future we can use other file opening tools or define them for ourselfs
/// TODO: develop this
use crate::config::Config;
use std::{ffi::OsStr, path::Path, process::Command};

pub fn open_file(config: &Config, file: &str) -> Result<(), String> {
    // disown xdg-open
    let filetype = Path::new(&file)
        .extension()
        .and_then(OsStr::to_str)
        .expect("Error while getting file extension");
    let opener = config.openers.get(filetype);
    match opener {
        Some(opener) => {
            Command::new(opener)
                .arg(file)
                .spawn()
                .expect("failed to execute process");
            Ok(())
        }
        None => {
            Command::new("xdg-open")
                .arg(file)
                .spawn()
                .expect("failed to execute process");
            Ok(())
        }
    }
}

/// here we define the open commands
/// first we use xdg-open to open the file
/// in future we can use other file opening tools or define them for ourselfs
/// TODO: develop this
use std::process::Command;

pub fn open_file(file: &str) -> Result<(), String> {
    // disown xdg-open
    let res = Command::new("xdg-open").arg(file).spawn();
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

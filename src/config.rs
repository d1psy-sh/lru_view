use serde::Deserialize;
use std::{collections::HashMap, env, fs};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub capacity: usize,
    pub batch_size: usize,
    pub openers: HashMap<String, String>,
}
impl Config {
    pub(crate) fn load() -> std::result::Result<Config, std::string::String> {
        let home = env::var("HOME");
        match home {
            Ok(home) => {
                let config_path = format!("{home}.config/lru_view/config.toml");
                let c = fs::read_to_string(config_path);
                match c {
                    Ok(c) => Ok(toml::from_str(&c).expect("Unable to parse config file")),
                    Err(e) => Err(format!("Unable to read config file: {e}")),
                }
            }
            Err(e) => Err(format!("Unable to get home dir: {e}")),
        }
    }
}

use crate::list::List;
use crate::lru::LRU;
use serde::{Deserialize, Serialize};
use std::{env, fs};
use toml;
// this is the main app where the acitons get organized

#[derive(Debug, Clone)]
pub struct App {
    config: Config,
    list: AppList,
    pub lru: LRU,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct AppList {
    items: Vec<Item>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Item {
    name: String,
    count: usize,
}

#[derive(Deserialize, Debug, Clone)]
struct Config {
    pub capacity: usize,
    pub batch_size: usize,
}

impl App {
    /// runs the app function that is called in the main and should do everything
    pub fn run(&mut self) -> Result<(), String> {
        // load the config
        self.load_config();
        // load the list
        self.load();

        // run the app use the lru to handle the data
        self.lru = LRU::new(self.config.capacity);
        // for the list we need a item struct which holds the count
        // here the lru gets initilized
        // TODO: this is spagetti code (I don't know how spaguetti is spelled)
        for item in &self.list.items {
            self.lru.items.insert(item.name.clone(), item.count);
        }
        // sync the display list with the map in the app
        let mut items: Vec<String> = vec![];
        for i in self.lru.items.keys() {
            items.push(i.clone());
        }
        let list: List = List::new(items);
        let item = list.prompt(self.config.batch_size);
        match item {
            Some(i) => {
                self.lru.update(&i);
            }
            None => {
                Err("No item choosen!".to_string())?;
            }
        }
        // save the list in a folder
        self.save();
        Ok(())
    }
    /// load the configs from the app
    fn load_config(&mut self) {
        let home = env::var("HOME".to_string());
        match home {
            Ok(home) => {
                let config_path = format!("{}/.config/lru_view/config.toml", home);
                let config = fs::read_to_string(config_path);
                match config {
                    Ok(config) => {
                        let config: Config =
                            toml::from_str(&config).expect("Unable to parse config file");
                        self.config = config;
                    }
                    Err(_) => {
                        println!("Unable to load config file\nGoing to default config");
                        self.config = Config {
                            capacity: 20,
                            batch_size: 10,
                        };
                    }
                }
            }
            Err(_) => {
                println!("Unable to get home directory");
                self.config = Config {
                    capacity: 20,
                    batch_size: 10,
                };
            }
        }
    }
    /// load the list from the toml file the counts where saved in
    fn load(&mut self) {
        // this is temporary find a path that is conventional in linux
        // and make it cross plattform for windows and mac
        let list = fs::read_to_string("./list.toml");
        match list {
            Ok(list) => {
                let list: AppList = toml::from_str(&list).expect("Unable to parse list file");
                self.list = list;
            }
            Err(_) => {
                println!("Unable to load list file\nGoing to default list");
                self.list = AppList { items: vec![] };
            }
        }
    }
    /// save the list back with the counts to a toml file somewhere
    fn save(&mut self) {
        let file = toml::to_string(&self.list).expect("Unable to serialize list");
        let res = fs::write("./list.toml", file);
        match res {
            Ok(_) => {}
            Err(_) => {
                // don't know whether panic is right here
                panic!("Unable to save list file");
            }
        }
    }
}

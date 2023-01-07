use crate::args::Args;
use crate::list::List;
use crate::lru::LRU;
use serde::{Deserialize, Serialize};
use std::{env, fs};
use toml;
// this is the main app where the acitons get organized

#[derive(Debug)]
pub struct App {
    config: Config,
    list: AppList,
    lru: LRU,
    args: Args,
}

// TODO: merge applist and list cuz is basically the same
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
    pub fn new() -> Self {
        Self {
            config: Config {
                capacity: 20,
                batch_size: 10,
            },
            list: AppList { items: vec![] },
            lru: LRU::new(20),
            args: Args::new(),
        }
    }

    /// runs the app function that is called in the main and should do everything
    pub fn run(&mut self) -> Result<(), String> {
        // load the config
        self.load_config();
        // load the list
        self.load();
        let res = self.handle_args();
        match res {
            Ok(_) => {
                // save the list
                self.save()
            }
            Err(e) => Err(e),
        }
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
                    }
                }
            }
            Err(_) => {
                println!("Unable to get home directory");
            }
        }
    }
    /// load the list from the toml file the counts where saved in
    fn load(&mut self) {
        // this is temporary find a path that is conventional in linux
        // and make it cross plattform for windows and mac
        let home = env::var("HOME".to_string());
        match home {
            Ok(home) => {
                let lru_path = format!("{}/.lru_list.toml", &home);
                let list = fs::read_to_string(&lru_path);
                match list {
                    Ok(list) => {
                        let list: AppList =
                            toml::from_str(&list).expect("Unable to parse list file");
                        self.list = list;
                    }
                    Err(_) => {
                        println!("Unable to load list file\nCreating default list file");
                        fs::write(
                            lru_path,
                            r#"[[items]]
                        name = <enter name of your first file with full path>
                        count = 0"#,
                        )
                        .expect("Unable to create list file");
                        self.load();
                        self.list = AppList { items: vec![] };
                    }
                }
            }
            Err(_) => {
                println!("Unable to get home directory");
            }
        }

        // run the app use the lru to handle the data
        self.lru = LRU::new(self.config.capacity);
        // for the list we need a item struct which holds the count
        // here the lru gets initilized
        // TODO: this is spagetti code (I don't know how spaguetti is spelled)
        for item in &self.list.items {
            self.lru.items.insert(item.name.clone(), item.count);
        }
    }
    /// save the list back with the counts to a toml file somewhere
    fn save(&mut self) -> Result<(), String> {
        let list: AppList = AppList {
            items: self
                .lru
                .items
                .iter()
                .map(|(k, v)| Item {
                    name: k.clone(),
                    count: *v,
                })
                .collect(),
        };
        let file = toml::to_string(&list).expect("Unable to serialize list");
        let home = env::var("HOME".to_string());
        match home {
            Ok(home) => {
                let lru_path = format!("{}/.lru_list.toml", &home);
                let res = fs::write(lru_path, file);
                match res {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Unable to save list file: {}", e)),
                }
            }
            Err(_) => Err("Unable to get home directory".to_string()),
        }
    }

    fn handle_args(&mut self) -> Result<(), String> {
        if self.args.clear {
            let home = env::var("HOME".to_string()).expect("couldn't find home env var");
            let lru_path = format!("{}.lru_list.toml", &home);
            let res = fs::remove_file(&lru_path);
            match res {
                Ok(_) => {
                    println!("List cleared {}", lru_path);
                    return Ok(());
                }
                Err(e) => {
                    println!("Unable to clear list");
                    return Err(format!("Unable to clear list: {}", e));
                }
            }
        }
        if self.args.test {
            // TODO: implement a test mode when it is necessary
            println!("Test mode");
            return Ok(());
        }
        // if file update lru else list prompt
        match &self.args.file {
            Some(file) => {
                // TODO: this does not seem right there has to be a nicer way
                // to do that
                self.lru.update(&String::from(
                    fs::canonicalize(file)
                        .expect("canonicalize failed!")
                        .to_str()
                        .expect("canonicalize failed in to_str()"),
                ));
                match crate::open::open_file(file) {
                    Ok(_) => self.save(),
                    Err(e) => Err(format!("Unable to open file: {}", e)),
                }
            }
            None => self.update_from_list(),
        }
    }

    fn update_from_list(&mut self) -> Result<(), String> {
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
                crate::open::open_file(&i)
            }
            None => Err("No item choosen!".to_string()),
        }
    }
}

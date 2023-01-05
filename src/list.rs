use serde::{Deserialize, Serialize};
use std::process::exit;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct List {
    items: Vec<String>,
}

// TODO: refactor this with string slices this is super uncool I think
impl List {
    pub fn prompt(&self, len: usize) -> Option<String> {
        let mut idx = 0;
        loop {
            if (idx + len) >= self.items.len() {
                match print_batch(&self.items[idx..]) {
                    Some(i) => {
                        return Some(self.items[idx + i].clone());
                    }
                    None => {
                        println!("\tOut of bounds!");
                        exit(0);
                    }
                }
            }
            // print a batch get the index of the item which is choosen
            // update the item is the cache
            // invoce the command for the view
            let item = print_batch(&self.items[idx..(idx + len)]);
            match item {
                Some(i) => {
                    return Some(self.items[idx + i].clone());
                }
                None => {
                    idx += len;
                }
            }
        }
        // print a batch get the index of the item which is choosen
        // update the item is the cache
        // invoce the command for the view
    }

    pub(crate) fn new(items: Vec<String>) -> List {
        List { items }
    }
}

fn print_batch(items: &[String]) -> Option<usize> {
    let len = items.len();
    std::process::Command::new("clear").status().expect("Unable to clear the terminal");
    for (idx, i) in items.iter().enumerate() {
        println!("\t{}: {}", idx, i);
    }
    println!("\n\n\tEnter a number, n for next or q to quit!\n\n");
    return get_input(len);
}

fn get_input(len: usize) -> Option<usize> {
    // get a number from the user
    let mut input: String = "".to_string();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input.trim() {
        "q" => {
            println!("\tByeBye");
            exit(0)
        }
        "n" => None,
        number => {
            let n = number.parse();
            match n {
                Ok(n) => {
                    if n < len {
                        return Some(n);
                    }
                    println!("\tPlease enter a number or n for next or q to quit!");
                    return get_input(len);
                }

                Err(_) => {
                    println!("\tPlease enter a number or n for next or q to quit!");
                    return get_input(len);
                }
            }
        }
    }
}

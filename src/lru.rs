use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LRU {
    pub capacity: usize,
    pub items: HashMap<String, usize>,
}

impl LRU {
    pub fn new(capacity: usize) -> Self {
        let items = HashMap::with_capacity(capacity);
        Self { capacity, items }
    }

    pub fn update(&mut self, key: &String) {
        // if the item is not changed so the item is the last item used
        // we do not have to count the other items up cuz it is proportional
        // if we have a new item we have to count up and throw an item out
        // if we take another item the last item has to be 1
        // check if item exists in cache
        match self.items.get(key) {
            Some(count) => {
                if *count != 0 {
                    for (k, v) in &mut self.items {
                        if *k == *key {
                            *v = 0;
                        } else {
                            *v += 1;
                        }
                    }
                }
            }
            None => {
                // insert new val and remove the val with the hightest count
                for v in self.items.values_mut() {
                    *v += 1;
                }
                if self.capacity <= self.items.len() {
                    let mut max: (String, usize) = ("".to_string(), 0);
                    for (k, v) in &self.items {
                        if *v > max.1 {
                            max.1 = *v;
                            max.0 = k.clone();
                        }
                    }
                    self.items.remove(&max.0);
                    self.items.insert(key.clone(), 0);
                } else {
                    self.items.insert(key.clone(), 0);
                }
            }
        }
        // move it to the front if it does
    }
}

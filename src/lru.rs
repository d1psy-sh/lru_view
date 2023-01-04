use std::collections::HashMap;

pub struct LRU {
    capacity: usize,
    length: usize,
    items: HashMap<String, usize>,
}

impl LRU {
    pub fn new(capacity: usize) -> Self {
        let items = HashMap::with_capacity(capacity);
        Self {
            capacity,
            length: 0,
            items,
        }
    }

    pub fn update(&mut self, key: &String) {
        // check if item exists in cache
        match self.items.get_mut(key) {
            Some(path) => {
                *path = 0;
            }
            None => {
                // insert new val and remove the val with the hightest count
                if self.capacity < self.length {
                    let mut max: (String, usize) = ("".to_string(), 0);
                    for (k, v) in &self.items {
                        if *v > max.1 {
                            max.1 = *v;
                            max.0 = k.clone();
                        }
                    }
                    self.items.remove(&max.0);
                    self.items.insert(key.clone(), 0);
                }
            }
        }
        for v in self.items.values_mut() {
            *v += 1;
        }
        // move it to the front if it does
    }
}

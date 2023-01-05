use lru_view::lru::LRU;


#[test]
fn add_item_test() {
    let mut map = LRU::new(3);
    map.update(&String::from("hello"));
    map.update(&String::from("world"));
    map.update(&String::from("foo"));
    map.update(&String::from("bar"));
    dbg!(&map);
    assert_eq!(map.items.len(), 3);
    assert_eq!(map.items.get("world").unwrap().to_owned(), 2);
    assert_eq!(map.items.get("foo").unwrap().to_owned(), 1);
    assert_eq!(map.items.get("bar").unwrap().to_owned(), 0);
}

#[test]
fn add_two_test() {
    let mut map = LRU::new(3);
    map.update(&String::from("hello"));
    map.update(&String::from("world"));
    map.update(&String::from("foo"));
    map.update(&String::from("bar"));
    map.update(&String::from("foo"));
    assert_eq!(map.items.get("bar").unwrap().to_owned(), 1);
    map.update(&String::from("bar"));
    dbg!(&map);
    assert_eq!(map.items.len(), 3);
    assert_eq!(map.items.get("world").unwrap().to_owned(), 4);
    assert_eq!(map.items.get("foo").unwrap().to_owned(), 1);
    assert_eq!(map.items.get("bar").unwrap().to_owned(), 0);
}

#[test]
fn get_item_test() {
    let mut map = LRU::new(3);
    map.update(&String::from("hello"));
    map.update(&String::from("world"));
    map.update(&String::from("foo"));
    map.update(&String::from("hello"));
    map.update(&String::from("world"));
    map.update(&String::from("foo"));
    map.update(&String::from("hello"));
    map.update(&String::from("hello"));
    dbg!(&map);
    assert_eq!(map.items.get("hello").unwrap().to_owned(), 0);
    assert_eq!(map.items.get("foo").unwrap().to_owned(), 1);
    assert_eq!(map.items.get("world").unwrap().to_owned(), 2);
}

// TODO: add more test

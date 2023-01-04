pub mod lru;
pub mod list;

fn main() {
    // make a new list
    // let items = vec!["1", "hello", "world", "foo", "bar", "baze", "one", "some", "else"];
    let items = vec!["hello", "world", "foo", "bar", "baze", "one", "some", "else"];
    let list = list::List::new(items.iter().map(|x| x.to_string()).collect());
    let res = list.prompt(4);
    println!("You choose: {:?}", res);
}

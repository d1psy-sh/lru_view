use lru_view::app::App;

fn main() {
    let mut app = App::new();
    let res = app.run();
    match res {
        Ok(_) => {
            println!("Done!");
            println!("Be FAST!");
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }
}

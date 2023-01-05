use crate::App;
fn main() {
    let app = App::new();
    let res = app.run();
match res {
        Ok(_) => {
            println!("Done!");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

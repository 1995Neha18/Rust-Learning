use std::env;

fn main() {
    if let Ok(user) = env::var("USERNAME") {
       println!("User: {}", user)
       
    }
    else {
      println!("USERNAME environment variable not found")
    }
}

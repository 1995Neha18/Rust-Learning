// ---------------- Example-02: Reading Environment Variables ------------------
// use std::env;

// fn main() {
//     if let Ok(user) = env::var("USERNAME") {
//        println!("User: {}", user)
       
//     }
//     else {
//       println!("USERNAME environment variable not found")
//     }
// }


// --------------------- Example-02: Setting Environment Variables --------------------

use std::env;
 fn main(){
    // set a custom variable.
    env::set_var("MY VARIABLE", "Hello, Rust!");

    if let Ok(value) = env::var("MY VARIABLE"){
       println!("MY VARIABLE: {}", value)
    }
    else {
      println!("MY VARIABLE environment variable not found")
    }
 }
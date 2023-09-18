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

// use std::env;
//  fn main(){
//     // set a custom variable.
//     env::set_var("MY VARIABLE", "Hello, Rust!");

//     if let Ok(value) = env::var("MY VARIABLE"){
//        println!("MY VARIABLE: {}", value)
//     }
//     else {
//       println!("MY VARIABLE environment variable not found")
//     }
//  }

// ------------------ Example-03: Command-Line Arguments ------------------

// use std::env;

// fn main() {
//     // Access command-line arguments
//     let args: Vec<String> = env::args().collect();

//     println!("Program name: {}", args[0]);

//     for (index, arg) in args.iter().enumerate() {
//         println!("Arg {}: {}", index, arg);
//     }
// }

// ---------------------------------------------------------------------------------------------------------


// ------------- Example-01: ------------------

use std::fs;

fn main() -> std::io::Result<()> {
    fs::write("foo.txt", "Lorem ipsum")?;
    fs::write("bar.txt", "dolor sit")?;
    Ok(())
}

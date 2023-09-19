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

// use std::fs;

// fn main() -> std::io::Result<()> {
//     fs::write("foo.txt", "Lorem ipsum")?;
//     fs::write("bar.txt", "dolor sit")?;
//     Ok(())
// }

// -------------- Example-02: ------------------------

// use std::fs::File;
// use std::io::BufReader;
// use std::io::prelude::*;

// fn main()-> std::io::Result<()>{
//     let file = File::open("foo.txt")?;
//     let mut buf_reader = BufReader::new(file);
//     let mut contents = String::new();
//     buf_reader.read_to_string(&mut contents)?;
//     assert_eq!("Hello World!", contents);
//     Ok(())
// }

// The read_to_string method is called on the BufReader.
// It reads the contents of the file into the contents string.
// The &mut contents part means that the data will be written into the contents string.
// Again, the ? operator is used to handle potential errors.

// ------------------- impl block usecases ---------------------

struct Person {
    name: String,
    age: u32,
}

impl Person {
    // Constructor method to create a new Person
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }

    fn display_info(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
}

fn main() {
    // Create a Person instance using the constructor
    let person1 = Person::new("Neha", 28);
    person1.display_info();
}

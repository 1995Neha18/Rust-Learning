// use std::fmt;
struct Person {
    name: String,
    age: u32,
}

// impl fmt::Display for Person {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Name: {}, Age: {}", self.name, self.age)
//     }
// }

fn main() {
    let person = Person {
        name: String::from("Bob"),
        age: 25,
    };
    println!("{}", person);
}

// -------------------------------------------------

// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u32,
// }

// fn main() {
//     let person = Person {
//         name: String::from("Bob"),
//         age: 25,
//     };
//     println!("{:?}", person);
// }
// -------------------------------------------------


// fn main() {
//  // Note that \t is the TAB character
//  let output = "N\tO\tI\tC\tE";
//  println!("Debug: {:?}", output);
//  println!("Display: {}", output);
// }



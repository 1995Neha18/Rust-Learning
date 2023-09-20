// mod comparison; // Import the comparison module from comparison.rs

// use std::cmp::Ordering; // Importing the Ordering enum from the standard library
// use comparison::compare_numbers; // Importing the compare_numbers function

// fn main() {
//     let number1 = 5;
//     let number2 = 10;

//     let comparison = compare_numbers(number1, number2);

//     match comparison {
//         Ordering::Less => println!("Number 1 is less than Number 2"),
//         Ordering::Equal => println!("Number 1 is equal to Number 2"),
//         Ordering::Greater => println!("Number 1 is greater than Number 2"),
//     }
// }

// ------------------- with closure ----------------------

use std::cmp::Ordering;

fn main() {
    let mut strings = vec!["banana", "cherry", "grape", "apple", "date"];

    // Sort the vector of strings using custom comparison logic
    strings.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    // Print the sorted strings
    for s in &strings {
        println!("{}", s);
    }
}

// -----------------------------------------------------

// use std::cmp::Ordering;

// fn compare_strings(a: &String, b: &String) -> Ordering {
//     a.partial_cmp(b).unwrap_or(Ordering::Equal)  // unwrap_or_default()
// }

// fn main() {
//     let mut strings = vec![
//         "apple".to_string(),
//         "banana".to_string(),
//         "cherry".to_string(),
//         "date".to_string(),
//         "grape".to_string(),
//     ];

//     strings.sort_by(compare_strings);

//     for s in &strings {
//         println!("{}", s);
//     }
// }

// -----------------------------------------------------

// use std::cmp::Ordering;

// fn main() {
//     let number = 42;
//     // let number2 = 50;
//     let string = "apple";

//     // let comparison = number.partial_cmp(&number2);
//     let comparison = number.partial_cmp(&string);

//     match comparison {
//         Some(ordering) => println!("Comparison result: {:?}", ordering),
//         None => println!("Comparison not applicable"),
//     }
// }

// The partial_cmp method in Rust's standard library is primarily used for
//comparing values when the ordering may not always be possible
//or straightforward due to various reasons(like comparing a number with a string).

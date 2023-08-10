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


use std::cmp::Ordering;

fn main() {
    let mut strings = vec![
     "banana".to_string(),
     "cherry".to_string(),
     "grape".to_string(),
     "apple".to_string(),
        "date".to_string(),
    ];

    // Sort the vector of strings using custom comparison logic
    strings.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    // Print the sorted strings
    for s in &strings {
        println!("{}", s);
    }
}




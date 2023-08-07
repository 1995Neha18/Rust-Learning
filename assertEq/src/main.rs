mod neha;

// fn main() {
//     neha::hello();
//     let my_name = "Neha";
//     assert!(my_name == "Neha");
//     assert_eq!(my_name, "Neha");
//     assert_ne!(my_name, "Neha");
// }

// assert! //  when the value is true
// assert_eq! // when the two values are equal
// assert_ne! // when the two values are not equal



// Import the macro from the macro_rules.rs file
#[macro_use]
mod macro_rules;

fn main() {
    let x = 5;
    let y = 10;
    
    let sum = add_numbers!(x, y); // Using the macro
    
    println!("Sum: {}", sum);

    neha::hello();
}


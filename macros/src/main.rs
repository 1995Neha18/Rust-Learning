// #[macro_export]

// macro_rules! square {
//    ($x:expr) => {
//        $x * $x
//    }
// }

// fn main(){
//    let x = 9;
//    println!("the square of {} is {}",x, square!(x))

// }

// --------------------------------------------------------------

// fn first_word(s: &String) -> &str {
// let bytes = s.as_bytes();

// for (i, &item) in bytes.iter().enumerate() {
//     if item == b' ' {
//         return &s[0..i];
//     }
// }

// &s[..]

//  let mut i = 0;

//   for item in s.chars() {

//    println!("{}", item);
//       if item == ' ' {
//           return &s[0..i];
//       }
//       i += 1;
//   }
//   &s[..]

// }

// fn main() {
//  let input_string = String::from("Hello, world!");
//  let first_word_result = first_word(&input_string);
//  println!("First word: {}", first_word_result);
// }

// fn main() {
//     // let s = "hello world";
//     // let bytes = s.as_bytes();
//     // // let number_iteration = numbers.iter();
//     // // println!("{:?}", number_iteration);

//     // for (i, &item) in bytes.iter().enumerate() {
//     //     println!("{} {}", i, item);
//     //     if item == b' ' {
//     //         println!("{} {}", i, item);
//     //     }
//     // }

//     let numbers = [2, 1, 17, 99, 34, 56];
//     let numbers_iterator = numbers.iter();
//     for (i,number) in numbers_iterator.enumerate() {
//      println!("{}-index {}", i,number);
//  }
// }

// -------------------------------------------------

// #[macro_export]

// macro_rules! square {
//     ($x:expr,$y:expr) => {
//          ($x + $y)*($x + $y)
//     }
// }

// fn main() {
//     let x = 9;
//     let y = 10;
//     println!("The sqaure of {}+ {} is {}", x,y, square!(x,y));

// }

// ------------- Repeat 3 times -----------------------------

// macro_rules! repeat_3_times {
//  ($expr:expr) => {
//      $expr; $expr; $expr;
//  };
// }

// fn main() {
//  repeat_3_times!(println!("Hello, Rust!"));
// }

// ---------------- Pattern Matching ----------------------------

// #[macro_export]
// macro_rules! match_example {
//  ($expr:expr) => {
//      match $expr {
//          0 => println!("Zero"),
//          1 => println!("One"),
//          10 => println!("Ten"),
//          _ => println!("Other"),
//      }
//  };
// }

// fn main() {
//  match_example!(0);
//  match_example!(1);
//  match_example!(10);
//  match_example!(42);
// }

// ----------------------------------------------

// pub mod my_macros;

// fn main() {
//     // Use the macro to add two numbers
//     let result: i32 = add_numbers!(5, 7);
//     println!("Result: {}", result);
// }

// --------------------------------------------

// macro_rules! platform_dependent_code {
//  () => {
//      #[cfg(target_os = "linux")]
//      println!("Running on Linux");

//      #[cfg(target_os = "windows")]
//      println!("Running on Windows");
//  };
// }

// fn main() {
//  platform_dependent_code!();
// }

// ----------------------- Creating Structs ----------------------------

// pub mod my_macros;

// struct Person {
//     name: String,
//     age: u32,
//     person_a: PersonA,
// }

// struct PersonA {
//     id: u32,
//     address: String,
//     name: String,
// }
// fn main() {
//     let person1 = create_person!("Alice", 30, 1, "123 Main St");
//     let person2 = create_person!("Bob", 25, 2, "456 Elm St");

//     println!("Name: {}, Age: {}", person1.name, person1.age);
//     println!("ID: {}, Address: {}", person1.person_a.id, person1.person_a.address);
//     println!("PersonA Name: {}", person1.person_a.name);

//     println!("Name: {}, Age: {}", person2.name, person2.age);
//     println!("ID: {}, Address: {}", person2.person_a.id, person2.person_a.address);
//     println!("PersonA Name: {}", person2.person_a.name);
// }



// fn main() { // immutable variable
//  let mut x = 5;
//  println!("The value of x is: {x}");
//  x = 6;
//  println!("The value of x is: {x}");
// }

// fn main() { // shadowing
//  let x = 5;

//  let x = x + 1;

//  {
//      let x = x * 2;
//      println!("The value of x in the inner scope is: {x}");
//  }

//  println!("The value of x is: {x}");
// }

// let guess: u32 = "42".parse().expect("Not a number!"); // type annotation

// fn main() { //tuple
//  let tup: (i32, f64, u8) = (500, 6.4, 1);
// }

// use std::io; // finding index of an array

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");
// }

// fn main() {
//  another_function(5);
// }

// fn another_function(x: i32) {
//  println!("The value of x is: {x}");
// }

// fn main() { // function with static type including integers and characters.
//  print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//  println!("The measurement is: {value}{unit_label}");
// }

// fn five() -> i32 {
//  5
// }

// fn main() {
//  let x = five();

//  println!("The value of x is: {x}");
// }

// fn main() {  // nested loop.
//  let mut count = 0;
//  'counting_up: loop {
//      println!("count = {count}");
//      let mut remaining = 10;

//      loop {
//          println!("remaining = {remaining}");
//          if remaining == 9 {
//              break;
//          }
//          if count == 2 {
//              break 'counting_up;
//          }
//          remaining -= 1;
//      }

//      count += 1;
//  }
//  println!("End count = {count}");
// }

// fn main() {  // with while loop
//  let a = [10, 20, 30, 40, 50];
//  let mut index = 0;

//  while index < 5 {
//      println!("the value is: {}", a[index]);

//      index += 1;
//  }
// }

//output: the value is: 10
// the value is: 20
// the value is: 30
// the value is: 40
// the value is: 50

// fn main() {  // same output with for loop.
//  let a = [10, 20, 30, 40, 50];

//  for element in a {
//      println!("the value is: {element}");
//  }
// }

// fn main() {     //output: 3! 2! 1! LIFTOFF!!!
//  for number in (1..4).rev() {
//      println!("{number}!");
//  }
//  println!("LIFTOFF!!!");
// }


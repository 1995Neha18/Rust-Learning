// #[macro_export]
// macro_rules! square {
//     ($x:expr) => {
//      $x * $x
//     };
// }

// fn main() {
//     let x = 9; // semicolon is mandatory after assigning a value to a variable, otherwise it will throw an error.
//     println!("The sqaure of {} is {}", x, square!(x));
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

#[macro_export]

macro_rules! square {
    ($x:expr,$y:expr) => {
         ($x + $y)*($x + $y)
    }
}

fn main() {
    let x = 9;
    let y = 10;
    println!("The sqaure of {}+ {} is {}", x,y, square!(x,y));
    
}



// use rand::prelude::*;

// pub fn random_num() {
//  let x: u8 = random();
//  println!("{}", x);

//  if random() { // generates a boolean
//      println!("Heads!");
// }
// }

// --------------------------------------------------

// use rand::Rng;

// pub fn random_num() {
//     let mut rng = rand::thread_rng();
//     let x: u8 = rng.gen();
//     println!("{}", x);

//     if rng.gen::<bool>() {
//         println!("Heads!");
//     } else {
//         println!("Tails!");
//     }
// }

// --------------------------------------

// use rand::Rng;

// pub fn random_num() {

//     let mut rng = rand::thread_rng();
//     //This line creates a mutable instance of a random number generator using rand::thread_rng(). 
//     //This generator is specific to the current thread and can be used to generate random numbers.

//     let n1: u8 = rng.gen();
//     let n2: u16 = rng.gen(); 

//     println!("Random u8: {}", n1);
//     println!("Random u16: {}", n2);
//     println!("Random u32: {}", rng.gen::<u32>());
//     println!("Random i32: {}", rng.gen::<i32>());
//     println!("Random float: {}", rng.gen::<f64>());
// }

// ------------ generating random number within a range -------------------

use rand::Rng; 

pub fn random_num() {
    // Generate random number in the range [0, 99]
    let num = rand::thread_rng().gen_range(0..100);
    println!("{}", num);
}


// use rand::prelude::*;

// pub fn random_num() {
//  let x: u8 = random();
//  println!("{}", x);

//  if random() { // generates a boolean
//      println!("Heads!");
// }
// }

use rand::Rng;

pub fn random_num() {
    let mut rng = rand::thread_rng();
    let x: u8 = rng.gen();
    println!("{}", x);

    if rng.gen::<bool>() {
        println!("Heads!");
    } else {
        println!("Tails!");
    }
}

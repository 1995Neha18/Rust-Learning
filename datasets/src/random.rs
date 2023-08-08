use rand::prelude::*;

pub fn random_num() {
 let x: u8 = random();
 println!("{}", x);

 if random() { // generates a boolean
     println!("Heads!");
}
}
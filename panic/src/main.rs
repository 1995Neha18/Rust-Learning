// fn drink(beverage: &str) {
//     // You shouldn't drink too much sugary beverages.
//     if beverage == "lemonade" {
//         panic!("AAAaaaaa!!!!");
//     }

//     println!("Some refreshing {} is all I need.", beverage);
// }

// fn main() {
//     drink("water");
//     drink("lemonade");
// }

// --------- same code but using Result instead of panic ----------------

// fn drink(beverage: &str) -> Result<(), &'static str> {
//  // You shouldn't drink too much sugary beverages.
//  if beverage == "lemonade" {
//      return Err("AAAaaaaa!!!!");
//  }

//  println!("Some refreshing {} is all I need.", beverage);
//  Ok(())
// }

// fn main() {
//  if let Err(error_msg) = drink("water") {
//      println!("Error: {}", error_msg);
//  }

//  if let Err(error_msg) = drink("lemonade") {
//      println!("Error: {}", error_msg);
//  }
// }

// ----------------- Iterator -----------------

 fn main () {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter(); // v1_iter is immutable

    for value in v1_iter {
        println!("Got: {}", value);
    }
 }


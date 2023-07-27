pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}


//Adding Custom Failure Messages

// pub fn greeting(name: &str) -> String {
//  String::from("Hello!")
// }


// #[cfg(test)]
// mod tests {
//  use super::*;

//      #[test]
//    fn greeting_contains_name() {
//         let result = greeting("Carol");
//         assert!(
//             result.contains("Carol"),
//             "Greeting did not contain name, value was `{}`",
//             result
//         );
//     }
// }

// -----------------------------

// pub struct Guess {
//  value: i32,
// }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 {
//             panic!("Guess value must be between 1 and 100, got {}.", value);
//         }

//         Guess { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//  use super::*;

//  #[test]
//  #[should_panic]
//  fn greater_than_100() {
//      Guess::new(200);
//  }
// }

// -------- shoul panic with a warning message --------

// --snip--

// impl Guess { 
//  pub fn new(value: i32) -> Guess {
//      if value < 1 {
//          panic!(
//              "Guess value must be greater than or equal to 1, got {}.",
//              value
//          );
//      } else if value > 100 {
//          panic!(
//              "Guess value must be less than or equal to 100, got {}.",
//              value
//          );
//      }

//      Guess { value }
//  }
// }

// #[cfg(test)]
// mod tests {
//  use super::*;

//  #[test]
//  #[should_panic(expected = "less than or equal to 100")]
//  fn greater_than_100() {
//      Guess::new(200);
//  }
// }

// -------------- Using Result<T, E> in Tests --------------

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() -> Result<(), String> {
//         if 2 + 2 == 4 {
//             Ok(())
//         } else {
//             Err(String::from("two plus two does not equal four"))
//         }
//     }
// }

// -------------- Controlling How Tests Are Run --------------

// pub fn add_two(a: i32) -> i32 {  // all these tests will run in parallel.
//  a + 2                           // cargo test -- --test-threads=1 (to run tests sequentially and not in parallel)
// }                                // cargo test one_hundred (to run only one test)

// #[cfg(test)]
// mod tests {
//  use super::*;

//  #[test]
//  fn add_two_and_two() {
//      assert_eq!(4, add_two(2));
//  }

//  #[test]
//  fn add_three_and_two() {
//      assert_eq!(5, add_two(3));
//  }

//  #[test]
//  fn one_hundred() {
//      assert_eq!(102, add_two(100));
//  }
// }

// -------------- Ignoring Some Tests Unless Specifically Requested --------------

// #[test]
// fn it_works() {
//     assert_eq!(2 + 2, 4);
// }

// #[test]
// #[ignore]
// fn expensive_test() {
//     // code that takes an hour to run
// }




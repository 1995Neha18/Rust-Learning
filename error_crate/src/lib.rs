
pub mod error {
 
 pub enum LinearError {
     // Defining specific error variants
     NotFound,
     InvalidInput,
 }

 // Define a Result type specific to your crate
 pub type Result<T> = std::result::Result<T, LinearError>;
}




// enum LinearError {
//  InvalidInput,
//  OtherError,
//  // ... other error variants
// }
// fn some_function() -> Result<i32> {
//  // ...
//  if condition {
//      Ok(42)
//  } else {
//      Err(LinearError::InvalidInput)
//  }
// }

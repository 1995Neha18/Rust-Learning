
pub mod error {
 
 pub enum LinearError {
     // Defining specific error variants
     NotFound,
     InvalidInput,
 }

 // Define a Result type specific to your crate
 pub type Result<T> = std::result::Result<T, LinearError>;
}

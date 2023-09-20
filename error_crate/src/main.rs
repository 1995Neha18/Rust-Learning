// Import the error module from the custom crate
use error_crate::error::{LinearError, Result};

// Define a function that returns your custom Result type
fn divide(a: i32, b: i32) -> Result<f64> {
    if b == 0 {
        return Err(LinearError::InvalidInput);
    }
    Ok(a as f64 / b as f64)
}

fn main() {
    match divide(10, 3) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => match err {
            LinearError::InvalidInput => println!("Invalid input"),
            _ => println!("An error occurred"),
        },
    }
}

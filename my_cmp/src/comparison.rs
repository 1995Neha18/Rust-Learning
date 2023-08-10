use std::cmp::Ordering;  // Importing the Ordering enum from the standard library

pub fn compare_numbers(a: i32, b: i32) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

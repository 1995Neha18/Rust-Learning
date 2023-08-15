
// #[macro_export]
// macro_rules! add_numbers {
//     ($x:expr, $y:expr) => {
//         $x + $y
//     };
// }

#[macro_export]
macro_rules! create_person {
 ($name:expr, $age:expr) => {
  Person {
      name: $name.to_string(),
      age: $age,
  }
 };
}
// #[macro_export]
// macro_rules! add_numbers {
//     ($x:expr, $y:expr) => {
//         $x + $y
//     };
// }

#[macro_export]
macro_rules! create_person {
    ($name:expr, $age:expr, $id:expr, $address:expr) => {
     Person {
         name: $name.to_string(),
         age: $age,
         person_a: PersonA {
             id: $id,
             address: $address.to_string(),
             name: $name.to_string(), // Add the name field here
         },
     }
    };
}

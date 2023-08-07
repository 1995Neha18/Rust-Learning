// mod neha;

// fn main() {
//     neha::hello();
//     let my_name = "Neha";
//     assert!(my_name == "Neha");
//     assert_eq!(my_name, "Neha");
//     assert_ne!(my_name, "Neha");
// }

// assert! //  when the value is true
// assert_eq! // when the two values are equal
// assert_ne! // when the two values are not equal

macro_rules! print_twice {
    ($expr:expr) => {
   println!("{:?} = {:?}",stringify!($expr), $expr);
   println!("{:?} = {:?}",stringify!($expr), $expr);
    };
}

fn main() {
    let x = 5;
    let y = 10;

    print_twice!(x + y);
    print_twice!(x * y);
}

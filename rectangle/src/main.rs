

// fn main(){
//    let width1 = 30;
//    let height1 = 50;

//    println!("The area of the recatangle is {} square pixels.",
//    area(width1,height1)
//   )
//  }

//   fn area(width:u32,height:u32) -> u32 {
//     width*height
//   }
// ------------------------------------------------

// fn main() {  // with tuple
//  let rect1 = (30, 50);

//  println!(
//      "The area of the rectangle is {} square pixels.",
//      area(rect1)
//  );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//  dimensions.0 * dimensions.1
// }

// --------------------------------------------------

// struct Rectangle {
//  width: u32,
//  height: u32,
// }

// fn main() {
//  let rect1 = Rectangle {
//      width: 30,
//      height: 50,
//  };

//  println!(
//      "The area of the rectangle is {} square pixels.",
//      area(&rect1)
//  );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//  rectangle.width * rectangle.height
// }

// -------------- Method -------------------------


// struct Rectangle {  // Method
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }

// ---------------------------------------

// ---------------------- Enum ---------------------

// enum IpAddrKind {
//  V4,
//  V6,
// }

// struct IpAddr {
//  kind: IpAddrKind,
//  address: String,
// }

// let home = IpAddr {
//  kind: IpAddrKind::V4,
//  address: String::from("127.0.0.1"),
// };

// let loopback = IpAddr {
//  kind: IpAddrKind::V6,
//  address: String::from("::1"),
// };
// -------------- another way ---------------
// enum IpAddr {
//  V4(u8, u8, u8, u8),
//  V6(String),
// }

// let home = IpAddr::V4(127, 0, 0, 1);

// let loopback = IpAddr::V6(String::from("::1"));


// ---------------------------------------

// enum Option<T> {  option enum that handles the null behaviour
//  None,
//  Some(T),
// }

// --------------- Absolute and Relative path -----------------

// mod front_of_house {
//  pub mod hosting {
//      pub fn add_to_waitlist() {}
//  }
// }

// pub fn eat_at_restaurant() {
//  // Absolute path
//  crate::front_of_house::hosting::add_to_waitlist();

//  // Relative path
//  front_of_house::hosting::add_to_waitlist();
// }
// ---------------------------------------------------

       // Concatenation

       // let s1 = String::from("tic"); concatenation with +.
       // let s2 = String::from("tac");
       // let s3 = String::from("toe");
   
       // let s = s1 + "-" + &s2 + "-" + &s3;
       // output : tic-tac-toe   

       // let s1 = String::from("tic"); concatenation with format!
       // let s2 = String::from("tac");
       // let s3 = String::from("toe");
   
       // let s = format!("{s1}-{s2}-{s3}");
        // output : tic-tac-toe


        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        
        impl Rectangle {
            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }








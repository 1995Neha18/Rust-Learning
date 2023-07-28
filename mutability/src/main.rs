// ------------ A simple program to understand mutability ------------
// fn main () {
//   let a = 3;
//   let mut b = a;
//   b = b + 1;
//   println!("a = {}, b = {}", a,b)
// }
// output :- a = 3, b = 4
// ------------------------------------------------------------------------

// Same thing but with strings.

fn main() {
    let s1 = String::from("hello");
    // let s2 = s1; (this will throw an error because we are trying to use s1 after we have moved it to s2.)
    let mut s2 = s1.clone();
    s2.push_str(", world!");
    println!("s1 = {}, s2 = {}", s1, s2);
}
// output :- error[E0382]: borrow of moved value: `s1`,
//this won't compile because we are trying to use s1 after we have moved it to s2.
// So we need to use clone method to copy the value of s1 to s2.
// --------------------------------------------------------------------------

// -------------- Using references to solve the problem ----------------

// fn main () {
//   let s1 = String::from("hello");
//   let s2 = &s1; // s2 is a reference to s1, so s2 doesn't own the value of s1.
//   println!("s1 = {}, s2 = {}", s1,s2);
// }
// --------------------------------------------------------

// ------------ Function that can change things -------------

// fn append_dot(t: &mut String) {
//     t.push_str(".");
// }

// fn main() {
//     let mut s = String::from("Hello");
//     append_dot(&mut s);
//     println!("{}", s);
// }

// ----------------------------------------------

// -------------- Dereferencing in primitive types ----------------

fn plus_one (x: &mut i32) {
 *x = *x + 1;
}

fn main () {
 let mut x = 10;
 plus_one(&mut x);
 println!("{}", x);
}
// ------------ A simple program to understand mutability ------------
fn main () {
  let a = 3;
  let mut b = a;
  b = b + 1;
  println!("a = {}, b = {}", a,b)
}
// output :- a = 3, b = 4
// ------------------------------------------------------------------------


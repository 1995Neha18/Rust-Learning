// use num_traits::Float; // 0.2.1

// fn main() {
//     let f1: f32 = 2.0;
//     let f2: f64 = 3.0;
//     let i1: i32 = 3;

//     println!("{:?}", sqrt(f1));
//     println!("{:?}", sqrt(f2));
//     println!("{:?}", sqrt(i1)); // error: the trait bound `{integer}: Float` is not satisfied
// }

// fn sqrt<T: Float>(input: T) -> T {
//     input.sqrt()
// }


// --------------------- for Integer ----------------

use num_traits::PrimInt;

trait Trait {
    fn y(self) -> Self;
}

impl<T: PrimInt> Trait for T {
    fn y(self) -> Self {
        println!("called");
        return self;
    }
}

fn main() {
    let x = 56;
    println!("{}", x.y());
}




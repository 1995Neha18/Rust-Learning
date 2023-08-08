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

// use num_traits::PrimInt;

// trait Num {
//     fn y(self) -> Self;
// }

// impl<T: PrimInt> Num for T {
//     fn y(self) -> Self {
//         println!("called");
//         return self;
//     }
// }

// fn main() {
//     let x = 56;
//     println!("{}", x.y());
// }

// --------------------------------------------------

use num_traits::Float;

trait Num {
    fn y(self) -> Self;
}

impl<T: Float> Num for T {
    fn y(self) -> Self {
        println!("called");
        return self;
    }
}

fn main() {
    let x:f32 = 56.2;
    println!("{}", x.y());
}






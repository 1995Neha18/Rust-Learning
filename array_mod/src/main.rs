use ndarray::{stack, Axis, arr1, arr2, Array1, Array2};

fn main() {
    // Create two 1-dimensional arrays
    let array1: Array1<i32> = arr1(&[1, 2, 3]);
    let array2: Array1<i32> = arr1(&[4, 5, 6]);

    // Stack the arrays vertically
    let stacked_array: Array2<i32> = stack(Axis(0), &[array1.view(), array2.view()]).unwrap();

    println!("Stacked Array:\n{}", stacked_array);
}

//[[1, 2, 3], [4, 5, 6]]: This is an array literal containing two nested arrays, creating a 2x3 matrix.
//&[[1, 2, 3], [4, 5, 6]]: This takes a reference to the array literal, 
//which is then passed as an argument to the arr2 function.
//arr2(&[[1, 2, 3], [4, 5, 6]]): This creates a 2-dimensional array (matrix) using the arr2 function.


use ndarray::{ stack, Axis, arr1, arr2, Array1, Array2 };

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


//In the context of the ndarray crate, the .view() method is used 
//to create a lightweight view or reference to an existing array. 
//It allows you to work with a subset or a specific view of the original array 
//without copying the data. This is particularly useful for avoiding unnecessary 
//memory allocation and improving performance.

//when using the stack function, you should use .view() to create 
//lightweight views of the arrays you're stacking. 

//arr1: A function provided by the ndarray crate to create a 1-dimensional array.
//arr2: A function provided by the ndarray crate to create a 2-dimensional array.
//stack: A function provided by the ndarray crate to stack arrays along a specified axis.
//Axis: An enum provided by the ndarray crate that represents an axis in an array.
//Array1: A type provided by the ndarray crate that represents a 1-dimensional array.
//Array2: A type provided by the ndarray crate that represents a 2-dimensional array.




//As I see it, the difference is mostly that ArrayView will make the function more flexible 
//– you can pass in parts of larger arrays, or an ArrayView created from a slice, 
//whereas the variant that takes a reference to Array can only be called 
//when you really have an Array of the desired size.



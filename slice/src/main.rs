// fn main() {
//     let arr = [1, 10, 20, 4, 5, 6];
//     // let slice = &arr[1..4];
//     // arr[1] = 100;
//     let slice = &arr[1..4];
//     // let slice = &mut arr[1..4];
//     // slice[0] = 100;
//     // arr[1] = 400;
//     println!("{:?}", slice);
// }

// fn main() {
//     // let mut arr = [1, 10, 20, 4, 5, 6];
   
    
//     // let slice = &arr[..];
    
    
//     // println!("{:?}", slice);
//     //  arr.clear();
//     // // Print the address and length of the slice
//     // println!("Pointer: {:?}", arr);
 
//     // println!("Length: {:?}", slice);
// }

fn first_word(s: &String) -> &str {
 let bytes = s.as_bytes();

 for (i, &item) in bytes.iter().enumerate() {
     if item == b' ' {
         return &s[0..i];
     }
 }

 &s[..]
}

fn main() {
 let s = String::from("hello world");

 let word = first_word(&s);

 
 println!("the first word is: {}", word);
 s.clear(); // error!
 println!("the first word is: {}", s);
}

// fn main() {
//  let num = 42;

//  // Create a reference that borrows the `num` variable
//  let reference = &num;

//  // Print the reference and the value it points to
//  println!("Reference: {:p}", &reference);
//  println!("Value: {}", *reference);
// }

//
// In Rust, {:p} is a format specifier used in the println!
//  macro to display the memory address (pointer value)
// of a variable or value






// Note: we can change the original array using slice, bcz slice is a reference to the original array,
//it's pointing towards the original array and it's not creating a new array neither storing the data inside the memory.
// So, if we are changing the slice, we are changing the original array.



// If the original data is stored on the stack (e.g., a local array or a stack-allocated variable),
//  the slice will point to that data on the stack.

// If the original data is stored on the heap (e.g., data created with Box or Vec),
// the slice will point to that data on the heap.

// Regardless of where the data is stored,
// the slice itself remains on the stack and has a fixed size,
// which makes it efficient and suitable for borrowing portions of collections without the need for deep copies.
// This design ensures that slices do not lead to memory leaks or other issues related to ownership and borrowing.

//String is a heap-allocated data structure,

//In Rust, String data type represents a dynamically allocated,
// growable UTF-8 encoded text, and it is stored on the heap.
// Unlike string slices (&str), which are lightweight and stored on the stack,
// String objects are used when you need a mutable,
// resizable text data structure that can change in size during runtime.

// When you create a String using String::from or other methods that allocate memory,
// the actual character data of the string (the sequence of bytes representing the UTF-8 encoded text)
// is allocated on the heap. The String type owns this memory,
// and it is automatically managed and deallocated when the String goes out of scope.

// -----------------------------------------------------------------------------------------

// In Rust, slices are a powerful and flexible way to work with portions of data,especially arrays and strings.
// While it's true that slices are usually stored on the stack,
// they don't contain the actual data but rather a reference to a contiguous portion of data stored elsewhere,
// typically on the heap.

// Here are the reasons why we use slices in Rust:  

// 1.**Borrowing Mechanism**: Slices allow borrowing a portion of a collection (array, vector, or string) without taking ownership.
// This borrowing mechanism ensures that multiple parts of a program can access and
// manipulate the same data without unnecessary copying or taking ownership,
// which is crucial for efficient and safe memory management.

// 2. **Efficiency**: Slices are lightweight and have a fixed size (a pointer and a length).
// They do not have any overhead compared to references.
// By using slices, we can work with portions of data without incurring the cost of copying entire data structures,
// as would be the case if we used `String` or `Vec`.

// 3. **Large Data**: While slices are usually stored on the stack,
// they can refer to data stored on the heap.
// This means that they can be used to work with large amounts of data without actually storing the data on the stack.
// The actual data resides in the heap, and the slice only holds a reference to it.

// 4. **Generic Functions**: Slices allow writing generic functions that can operate on various collections without knowing their exact size at compile time.
// This enables more reusable code and avoids code duplication.

// 5. **Interoperability**: Many Rust APIs use slices as a common way to pass data between functions or modules.
// This makes it easy to interface with other parts of the Rust ecosystem or libraries that expect slices as input or return slices as output.

// 6. **Safety**: Slices are checked at runtime to ensure that they remain valid and do not outlive the data they reference.
// This helps to prevent common memory-related bugs like accessing invalid memory or causing undefined behavior.

// It's important to note that while slices themselves are small and efficient,
// they should be used judiciously for large data when the data itself is stored on the heap.
// If you need to work with very large datasets,
// consider using `Vec` or `Box` (smart pointer) to manage the memory on the heap explicitly.
// In cases where a slice becomes too large to fit on the stack,
// you can consider using a boxed slice (`Box<[T]>`) to store the data on the heap.


// ------------------ Why direct indexing is not possible to access elements in string ------------------------------
Rust allows indexing individual characters of a string using the bracket syntax. However,
this operation is not O(1) time complexity, as it might seem. Rust strings are encoded as UTF-8 by default,
and UTF-8 is a variable-length encoding, meaning that characters can have different byte lengths.
Indexing characters directly in a UTF-8 string requires traversing the string from the beginning
to the desired character, which is a potentially costly operation.


//----------------- Difference between String, &String and &str ------------------------------

1.&str is a borrowed reference to a sequence of UTF-8 bytes and is immutable by default.
2.&String is a borrowed reference to the whole String object and can be used
for both read and write access to the String.
3.String is an owned string and allows you to manage a mutable, dynamically sized string on the heap.
When deciding which one to use, consider whether you need to own the string,
whether you need to modify it, and what kind of interface you want to provide to your functions. 
String slices (&str) are often used for generic and flexible APIs, 
while String is used when you need to manage and modify the string's content.


1.&str (String Slice):

&str is a string slice, which is a borrowed view into a sequence of UTF-8 bytes.
It represents a reference to a piece of string data owned by another data structure, 
such as a String or a string literal (&'static str).
String slices are typically used for borrowing string data without taking ownership.
String slices are immutable by default and cannot be modified. 
To modify the content of a string slice, you need to convert it into a mutable slice (&mut str), 
but that's not directly allowed because string literals are usually stored in read-only memory.

2.&String (Reference to String):

&String is a reference to the entire String data structure, 
which itself owns the UTF-8 encoded string data on the heap.
It represents a borrowed reference to the whole String object.
Since a String can be mutable, a &String reference can be used to modify the content of the underlying String.
This is useful when you want to pass a reference to a String 
to a function or when you need to avoid ownership transfer of the String.

3.String (Owned String):

String is a data structure that owns a sequence of UTF-8 bytes representing a string on the heap.
It is used when you need to own and manage a dynamically sized, mutable string.
You can modify the content of a String because you have ownership of it.
You can convert a String to a &str by using the & operator, 
which borrows the string data without transferring ownership.


Use Cases:

Use &str when you want your function to accept different types of string data (string literals, 
string slices, and String objects) and you don't need to modify the content of the string. 
This makes your functions more generic and reusable.
Use &String when you explicitly need a reference to a String object (e.g., to modify its content) 
or when you want to communicate that the function expects ownership of a String.

String::from("hello world")

---------------------------------------------------------------------

In Rust, the Cargo.toml and Cargo.lock files are used by the Cargo build system 
and package manager to manage dependencies and project metadata.

1. Cargo.toml:
The Cargo.toml file is the manifest file for a Rust project. 
It is written in the TOML (Tom's Obvious, Minimal Language) format.
It is located at the root of the project directory and contains 
important information about the project, including its name, version, 
dependencies, build options, and other metadata.
The Cargo.toml file is used to declare the project's dependencies, 
which are external crates (Rust packages) that the project relies on.
You can also specify the project's version, authors, description, license, 
and various other project-specific details in the Cargo.toml file.

2.Cargo.lock:
The Cargo.lock file is automatically generated by Cargo and is used to lock the versions of dependencies 
used in the project.
It records the exact versions of the dependencies and their transitive dependencies 
that were resolved by Cargo during the last successful build or cargo update command.
The Cargo.lock file ensures that all developers and build environments use the exact 
same versions of dependencies, providing deterministic builds and avoiding version conflicts.
When you run cargo build or cargo run, Cargo checks the Cargo.lock file and 
installs the exact specified versions of dependencies.














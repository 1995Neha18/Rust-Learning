// use reqwest::Client;
// use serde::{ Deserialize, Serialize };

// #[derive(Debug, Deserialize, Serialize)]
// struct Todo {
//     #[serde(rename = "userId")]
//     user_id: i32,
//     id: Option<i32>,
//     title: String,
//     completed: bool,
// }

// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error> {
//     let updated_todo = Todo {
//         user_id: 1,
//         id: Some(1),
//         title: "Updated Title".to_string(),
//         completed: true,
//     };

//     let updated_todo: Todo = Client::new()
//         .put("https://jsonplaceholder.typicode.com/todos/1") // Assuming you want to update todo with ID 1
//         .json(&updated_todo)
//         .send().await?
//         .json().await?;

//     println!("{:#?}", updated_todo);

//     // ------------------------------------------------------------

//     let updated_fields = serde_json::json!({
//      "title": "Hello Neha!".to_owned()
//  });

//     let updated_todo: Todo = Client::new()
//         .patch("https://jsonplaceholder.typicode.com/todos/1") // Assuming you want to patch todo with ID 1
//         .json(&updated_fields)
//         .send().await?
//         .json().await?;

//     println!("{:#?}", updated_todo);

//     // ----------------- Delete Request ----------------------

//     // let response = Client::new()
//     //     .delete("https://jsonplaceholder.typicode.com/todos/1") // Assuming you want to delete todo with ID 1
//     //     .send().await?;

//     // println!("Response: {:?}", response);

//     Ok(())
// }

// ----------------------------------------

// mod hashmap;

// fn main() {
//     hashmap::hashmap_value();
// }

// ---------------------------------------------

// use std::collections::HashMap;

// fn get_hashmap() -> HashMap<u32, &'static str> {
//     let mut m = HashMap::new();
//     m.insert(0, "foo");
//     m.insert(1, "bar");
//     m.insert(2, "baz");
//     m
// }

// fn main() {
//     // First access to the HashMap initializes it
//     let hashmap = get_hashmap();
//     println!("The entry for `0` is \"{}\".", hashmap.get(&0).unwrap());

//     // Any further access to the HashMap just returns the computed value
//     println!("The entry for `1` is \"{}\".", hashmap.get(&1).unwrap());
// }

// ----------------------------------------------------------------------------

// use std::collections::HashMap;

// fn main() {
//     let mut hashmap = HashMap::new();

//     // insert() method
//     hashmap.insert("key1", "value1");
//     hashmap.insert("key2", "value2");

//     // get() method
//     if let Some(value) = hashmap.get("key1") {
//         println!("Value for key 'key1': {}", value);
//     }

//     // remove() method
//     if let Some(removed_value) = hashmap.remove("key2") {
//         println!("Removed value for key 'key2': {}", removed_value);
//     }

//     // contains_key() method
//     if hashmap.contains_key("key1") {
//         println!("The map contains 'key1'.");
//     } else {
//         println!("The map does not contain 'key1'.");
//     }

//     // len() method
//     println!("Number of key-value pairs in the map: {}", hashmap.len());

//     // is_empty() method
//     if hashmap.is_empty() {
//         println!("The map is empty.");
//     } else {
//         println!("The map is not empty.");
//     }

//     // keys() method
//     for key in hashmap.keys() {
//         println!("Key: {}", key);
//     }

//     // values() method
//     for value in hashmap.values() {
//         println!("Value: {}", value);
//     }

//     // iter() method
//     for (key, value) in hashmap.iter() {
//         println!("Key: {}, Value: {}", key, value);
//     }

//     // entry() method
//     let entry = hashmap.entry("key3").or_insert("default");
//     println!("Value for key 'key3': {}", entry);

//     // clear() method
//     hashmap.clear();
//     println!("After clearing the map, is it empty? {}", hashmap.is_empty());
// }

// -------------- Using the DateTime Struct -----------------------------

// use chrono::prelude::*;

// fn main() {
//     let utc: DateTime<Utc> = Utc::now();
//     let local: DateTime<Local> = Local::now();

//     println!("UTC now is: {}", utc);
//     println!("Local now is: {}", local);
// }

// -------------------- Creating Unique ID -----------------------------

// use uuid::Uuid;

// fn main() {
//     let id = Uuid::new_v4();
//     println!("{}", id);
// }

//If you have a UUID value, you can use its string literal form inline:

use uuid::{Uuid,uuid};

fn main()
{
   const ID:Uuid = uuid!("936DA01F9ABD4d9d80C702AF85C822A8");
   println!("{}",ID);
}
// Serialization: This process involves converting structured data,
//such as Rust structs, enums, or other data types, into a format that can be easily stored or transmitted.
//In the context of serde, serialization typically produces a JSON, XML,

// Deserialization: This process is the reverse of serialization.
// It involves converting the serialized data (e.g., JSON) back into the original Rust data structures.

// use serde::{ Deserialize, Serialize };
// use reqwest::Client;
// use std::error::Error;

// #[derive(Debug, Serialize, Deserialize)] //
// struct Todo {
//     #[serde(rename = "userId")]
//     user_id: i32,
//     id: Option<i32>,
//     title: String,
//     completed: bool,
// }

// #[tokio::main]

// // calling an API and getting the response in json then converting it into a vector of Todo or (struct)
// async fn main() -> Result<(), Box<dyn Error>> {  // dyn: dynamically dispatched trait object
//     let response = Client::new()
//         .get("https://jsonplaceholder.typicode.com/todos")
//         .send().await?;

//     if response.status().is_success() {
//         let todos: Vec<Todo> = response.json().await?;
//         // let todos: String = response.text().await?;

//         println!("{:#?}", todos);
//     } else {
//         eprintln!("Request failed with status: {}", response.status());
//     }

//     // --------------- taking rust type converting into json and sending it to an API ----------------

// const new_todo: Todo = Todo {
//     user_id: 1,
//     id: None,
//     title: "Learn Rust".to_owned(),
//     completed: false,
// };

// let new_todo: Todo = reqwest::Client::new()
//     .post("https://jsonplaceholder.typicode.com/todos")
//     .json(&new_todo) // converting the rust type into json or serializing the data
//     .send().await?
//     .json().await?;

// println!("{:#?}", new_todo);

//         Send and receive arbitrary JSON

//         let new_todo: serde_json::Value = reqwest::Client
//         ::new()
//         .post("https://jsonplaceholder.typicode.com/todos")
//         .json(
//             &serde_json::json!({
//         "userId": 1,
//         "title": "Hello World".to_owned(),
//         "completed": false
//     })
//         )
//         .send().await?
//         .json().await?;

//     println!("{:#?}", new_todo);

//     Ok(())
// }

// serde_json = "1.0" // it gives us the access to the macro that allows us to construct arbitrary JSON objects.

// use std::error::Error;
// mod neha;

// fn main() -> Result<(), Box<dyn Error>> {
//     neha::fetch_todo()
// }

use reqwest::{ Client, Error };
use serde::{ Deserialize, Serialize };
use std::error::Error as StdError;

#[derive(Debug, Deserialize, Serialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() {
    let url = "https://jsonplaceholder.typicode.com/todos?userID=1"; // Change this URL as needed
    let expected_url = "https://jsonplaceholder.typicode.com/todos";

    if url != expected_url {
        eprintln!("Error: URL does not match expected URL");
        return;
    }

    let client = reqwest::Client::new();
    let response = client.get(url).send().await;

    match response {
        Ok(response) => {
            if response.status().is_success() {
                let json_response: serde_json::Value = response.json().await.unwrap();
                println!("JSON Response: {:#?}", json_response);
            } else {
                eprintln!("Request failed with status: {}", response.status());
            }
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
}

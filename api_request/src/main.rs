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

mod hashmap;

fn main() {
    hashmap::hashmap_value();
}


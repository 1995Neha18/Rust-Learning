// fn main() {
//     println!("Hello, world!");
// }

// use conmmand: cargo run in the terminal to run the program in the terminal.
// use command: cargo build to build the program.This command creates an executable file
// in target/debug/hello_cargo
// use command: cargo check to check whether the program can be compiled successfully,
//but it does not generate an executable file.

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Receive type-checked JSON
    
    let todos: Vec<Todo> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", todos);

     // Send and receive type-checked JSON

     let new_todo = Todo {
      user_id: 1,
      id: None,
      title: "Subscribe to Let's Get Rusty".to_owned(),
      completed: false
  };

  let new_todo: Todo = reqwest::Client::new()
      .post("https://jsonplaceholder.typicode.com/todos")
      .json(&new_todo)
      .send()
      .await?
      .json()
      .await?;

  println!("{:#?}", new_todo);

  Ok(())
}

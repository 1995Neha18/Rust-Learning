use reqwest::Client;
use serde::{ Deserialize, Serialize };
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)] //
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

#[tokio::main]
pub async fn fetch_todo() -> Result<(), Box<dyn Error>> {
    let response = Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send().await?;

    if response.status().is_success() {
        // let todos: Vec<Todo> = response.json().await?;
        let todos: Vec<Todo> = response.json().await?;

        println!("{:#?}", todos);
    } else {
        eprintln!("Request failed with status: {}", response.status());
    }
    Ok(())
}

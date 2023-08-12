use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let updated_todo = Todo {
        user_id: 1,
        id: Some(1),
        title: "Updated Title".to_string(),
        completed: true,
    };

    let updated_todo: Todo = Client::new()
        .put("https://jsonplaceholder.typicode.com/todos/1") 
        .json(&updated_todo)
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", updated_todo);
    Ok(())
}

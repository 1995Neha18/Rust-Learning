use reqwest::Client;
use serde::{ Deserialize, Serialize };

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
    let updated_fields = serde_json::json!({
    "title": "Hello Neha!".to_owned()
});

    let updated_todo: Todo = Client::new()
        .patch("https://jsonplaceholder.typicode.com/todos/1")
        .json(&updated_fields)
        .send().await?
        .json().await?;

    println!("{:#?}", updated_todo);
    Ok(())
}

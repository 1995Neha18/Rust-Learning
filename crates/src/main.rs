use serde::{Deserialize, Serialize}; // it is used to serialize and deserialize the data

#[derive(Debug, Serialize, Deserialize)] // 
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

#[tokio::main]

// calling an API and getting the response in json then converting it into a vector of Todo or (struct)
async fn main() -> Result<(), reqwest::Error> {
     let todos: Vec<Todo> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send()
        .await? // await the response
        .json() // turn the response in json
        //.text()  // turn the response in string
        .await?; // await the operation

     println!("{:#?}",todos);

     Ok(())
}

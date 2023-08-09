use serde::{ Deserialize, Serialize }; // it is used to serialize and deserialize the data

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
    let todos: Vec<Todo> = reqwest::Client
        ::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send().await
        ? // await the response
        .json().await?; // turn the response in json
    //.text()  // turn the response in string // await the operation
    println!("{:#?}", todos);

    // --------------- taking rust type converting into json and sending it to an API ----------------

    let new_todo: Todo = Todo {
        user_id: 1,
        id: None,
        title: "Learn Rust".to_owned(),
        completed: false,
    };

    let new_todo: Todo = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&new_todo) // converting the rust type into json or serializing the data
        .send().await?
        .json().await?;

    println!("{:#?}", new_todo);

      // Send and receive arbitrary JSON

      let new_todo: serde_json::Value = reqwest::Client
      ::new()
      .post("https://jsonplaceholder.typicode.com/todos")
      .json(
          &serde_json::json!({
      "userId": 1,
      "title": "Hello World".to_owned(),
      "completed": false
  })
      )
      .send().await?
      .json().await?;

  println!("{:#?}", new_todo);

    Ok(())
}

// serde_json = "1.0" // it gives us the access to the macro that allows us to construct arbitrary JSON objects.

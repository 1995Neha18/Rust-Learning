use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let response = Client::new()
        .delete("https://jsonplaceholder.typicode.com/todos/1") // Assuming you want to delete todo with ID 1
        .send()
        .await?;

    println!("Response: {:#?}", response);
    Ok(())
}

// #[tokio::main] // procedural macro attribute.


// async fn main() {
//     let f = my_function();
//     println!("Hello, world!");
//     f.await;
// }

// async fn my_function() {
//     println!("I'm an async function!");
//     let s1: String = read_from_database().await;
//     println!("First result: {}", s1);
//     let s2: String = read_from_database().await;
//     println!("Second result: {}", s2);
// }

// async fn read_from_database() -> String {
//     "DB RESULT".to_owned()
// }

use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let task1 = async_task_1();
    let task2 = async_task_2();

    tokio::join!(task1, task2);
}

async fn async_task_1() {
    tokio::time::sleep(Duration::from_secs(4)).await;
    println!("Async Task 1 completed");
}

async fn async_task_2() {
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("Async Task 2 completed");
}

// Async/Await in Rust
// ====================
// 🔥 CRITICAL untuk Backend Development!

use std::time::Duration;

// ================================
// BASIC ASYNC FUNCTION
// ================================

async fn hello_async() {
    println!("Hello from async function!");
}

async fn fetch_data(id: u32) -> String {
    // Simulate network delay
    tokio::time::sleep(Duration::from_millis(100)).await;
    format!("Data for id: {}", id)
}

// ================================
// CONCURRENT EXECUTION
// ================================

async fn fetch_all() {
    // Sequential (slow)
    let data1 = fetch_data(1).await;
    let data2 = fetch_data(2).await;
    println!("Sequential: {}, {}", data1, data2);

    // Concurrent (fast) - using tokio::join!
    let (data1, data2, data3) = tokio::join!(
        fetch_data(1),
        fetch_data(2),
        fetch_data(3)
    );
    println!("Concurrent: {}, {}, {}", data1, data2, data3);
}

// ================================
// ASYNC WITH RESULT
// ================================

async fn fetch_user(id: u32) -> Result<String, String> {
    if id == 0 {
        return Err("Invalid user id".to_string());
    }
    tokio::time::sleep(Duration::from_millis(50)).await;
    Ok(format!("User {}", id))
}

async fn get_users() -> Result<Vec<String>, String> {
    let user1 = fetch_user(1).await?;
    let user2 = fetch_user(2).await?;
    Ok(vec![user1, user2])
}

// ================================
// SPAWNING TASKS
// ================================

async fn spawn_example() {
    // Spawn a task that runs in background
    let handle = tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        "Task completed!"
    });

    println!("Task spawned, doing other work...");
    
    // Wait for task to complete
    let result = handle.await.unwrap();
    println!("Result: {}", result);
}

// ================================
// TOKIO SELECT
// ================================

async fn select_example() {
    tokio::select! {
        _ = tokio::time::sleep(Duration::from_millis(100)) => {
            println!("100ms elapsed");
        }
        _ = tokio::time::sleep(Duration::from_millis(200)) => {
            println!("200ms elapsed");
        }
    }
    // Only one branch executes (the first to complete)
}

// ================================
// ASYNC ITERATOR (Stream)
// ================================
// Requires: tokio-stream = "0.1"

// async fn stream_example() {
//     use tokio_stream::{self as stream, StreamExt};
//     
//     let mut stream = stream::iter(vec![1, 2, 3]);
//     while let Some(value) = stream.next().await {
//         println!("Got: {}", value);
//     }
// }

// ================================
// MAIN WITH TOKIO RUNTIME
// ================================

#[tokio::main]
async fn main() {
    println!("=== Async Rust Demo ===\n");

    // Basic async
    hello_async().await;

    // Fetch data
    let data = fetch_data(42).await;
    println!("Fetched: {}", data);

    // Concurrent execution
    println!("\n--- Concurrent Execution ---");
    fetch_all().await;

    // Error handling
    println!("\n--- Async with Result ---");
    match get_users().await {
        Ok(users) => println!("Users: {:?}", users),
        Err(e) => println!("Error: {}", e),
    }

    // Spawning tasks
    println!("\n--- Spawn Tasks ---");
    spawn_example().await;

    // Select
    println!("\n--- Select ---");
    select_example().await;

    println!("\nDone!");
}

// ================================
// PRACTICAL EXAMPLE: HTTP Client
// ================================
// Requires: reqwest = { version = "0.11", features = ["json"] }

// async fn http_example() -> Result<(), reqwest::Error> {
//     let resp = reqwest::get("https://api.github.com/users/octocat")
//         .await?
//         .json::<serde_json::Value>()
//         .await?;
//     println!("Response: {}", resp);
//     Ok(())
// }

// ================================
// CARGO.TOML DEPENDENCIES
// ================================
/*
[dependencies]
tokio = { version = "1", features = ["full"] }
# Optional:
# reqwest = { version = "0.11", features = ["json"] }
# tokio-stream = "0.1"
*/

// EXERCISES:
// 1. Buat fungsi async yang fetch data dari 3 API secara concurrent
//    dan return hasilnya dalam satu struct
//
// 2. Implementasikan timeout untuk async operation menggunakan
//    tokio::time::timeout
//
// 3. Buat simple async task queue yang process items satu per satu

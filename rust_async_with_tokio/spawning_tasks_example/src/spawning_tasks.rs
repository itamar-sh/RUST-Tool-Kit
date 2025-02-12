use std::{thread, time};  // Non async code

use tokio::time::{sleep, Duration};  // Async code

// Non async code
fn blocking_call() -> String {
    thread::sleep(time::Duration::from_secs(5));

    "Finally done".to_string()
}

// Async code function
async fn async_call(id: i32) {
    sleep(Duration::from_secs(1)).await;
    println!("Async Call: ID {}", id);
}

// Async code main function
#[tokio::main]
async fn main() {
    let blocking_code_handle = tokio::task::spawn_blocking(blocking_call);

    let mut async_handles = Vec::new();
    for id in 0..10 {
        async_handles.push(tokio::spawn(async_call(id)));
    }

    for handle in async_handles {
        handle.await.unwrap();
    }

    let result = blocking_code_handle.await.unwrap();
    println!("Blocking call: {}", result);
}

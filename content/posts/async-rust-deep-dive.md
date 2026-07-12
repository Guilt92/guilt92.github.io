# Async Rust: Futures, Tasks, and Executors

Understanding the async/await model in Rust from the ground up: how futures work, task scheduling, and the Tokio runtime.

## What is a Future?

A Future in Rust is a value that represents a computation that may not be ready yet. It's a state machine that progresses from `Pending` to `Ready(T)`:

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct Delay {
    dur: std::time::Duration,
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // Check if timer has elapsed
        // If not, register a waker and return Poll::Pending
        // If yes, return Poll::Ready(())
        Poll::Pending
    }
}
```

## async/await Syntax

`async fn` is syntactic sugar for functions that return `impl Future`:

```rust
// These are equivalent:
async fn fetch_data(url: &str) -> String { ... }

fn fetch_data(url: &str) -> impl Future<Output = String> + '_ { ... }
```

Every `.await` point is a potential suspension point where the runtime can switch to another task.

## The Executor

Futures don't run themselves. An executor polls them to make progress:

```rust
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let data = fetch_data("https://api.example.com").await;
        println!("Got: {}", data);
    });
}
```

## Task spawning

Tokio lets you run futures concurrently:

```rust
use tokio::task;

async fn process() {
    // Spawn independent tasks
    let handle1 = task::spawn(async {
        fetch_user(1).await
    });

    let handle2 = task::spawn(async {
        fetch_user(2).await
    });

    // Await both concurrently
    let (user1, user2) = tokio::join!(handle1, handle2);
}
```

## Channels for Communication

```rust
use tokio::sync::mpsc;

async fn producer(sender: mpsc::Sender<i32>) {
    for i in 0..10 {
        sender.send(i).await.unwrap();
    }
}

async fn consumer(mut receiver: mpsc::Receiver<i32>) {
    while let Some(value) = receiver.recv().await {
        println!("Received: {}", value);
    }
}

let (tx, rx) = mpsc::channel(32);
tokio::spawn(producer(tx));
tokio::spawn(consumer(rx));
```

## Common Patterns

### Timeout

```rust
use tokio::time::{timeout, Duration};

match timeout(Duration::from_secs(5), fetch_data(url)).await {
    Ok(data) => println!("Got data: {}", data),
    Err(_) => eprintln!("Request timed out"),
}
```

### Select

```rust
use tokio::select;

tokio::select! {
    data = fetch_primary() => handle_primary(data),
    fallback = fetch_fallback() => handle_fallback(fallback),
    _ = tokio::time::sleep(Duration::from_secs(3)) => {
        eprintln!("All sources timed out");
    }
}
```

## Pitfalls

### Blocking the Runtime

Never use `std::thread::sleep` or blocking I/O in async code:

```rust
// BAD: Blocks the entire runtime thread
async fn bad() {
    std::thread::sleep(Duration::from_secs(1));
}

// GOOD: Yields to the runtime
async fn good() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

### Send Bounds

Tokio tasks must be `Send` — they can move between threads. Be careful with `Rc`, raw pointers, or `*mut T` in async blocks.

## Conclusion

Async Rust gives you the performance of event-driven I/O with the ergonomics of synchronous-looking code. The key insight: futures are lazy — they only make progress when polled by an executor. Understanding this mental model makes everything else click.

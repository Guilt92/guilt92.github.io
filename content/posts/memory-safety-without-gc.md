# Memory Safety Without Garbage Collection

Deep dive into Rust's ownership model, borrow checker, and how they eliminate entire classes of bugs at compile time.

## The Problem with Garbage Collection

GC-managed languages (Java, Go, JavaScript) trade runtime performance for memory safety. The garbage collector:

- Adds latency spikes during collection cycles
- Consumes extra memory (typically 2-10x live data)
- Makes real-time systems unpredictable

## Rust's Approach: Ownership

Every value in Rust has exactly one **owner**. When the owner goes out of scope, the value is dropped (freed) immediately — no GC needed.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is MOVED to s2
    // println!("{}", s1);  // Compile error! s1 is no longer valid
    println!("{}", s2);     // This works
}
```

## Borrowing Rules

References let you access data without taking ownership:

- You can have **either** many immutable references (`&T`) **or** one mutable reference (`&mut T`)
- References must always be valid (no dangling pointers)

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
    // s goes out of scope here, but since it doesn't own the value,
    // nothing happens to the String
}

fn append_greeting(s: &mut String) {
    s.push_str(", world!");
}
```

## Lifetimes

Lifetimes ensure references don't outlive the data they point to:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

## Practical Example: Safe Buffer Management

```rust
struct Buffer {
    data: Vec<u8>,
    position: usize,
}

impl Buffer {
    fn new(capacity: usize) -> Self {
        Buffer {
            data: vec![0; capacity],
            position: 0,
        }
    }

    fn write(&mut self, bytes: &[u8]) -> usize {
        let available = self.data.len() - self.position;
        let to_write = bytes.len().min(available);
        self.data[self.position..self.position + to_write]
            .copy_from_slice(&bytes[..to_write]);
        self.position += to_write;
        to_write
    }

    fn read(&self, offset: usize, len: usize) -> &[u8] {
        let end = (offset + len).min(self.data.len());
        &self.data[offset..end]
    }
}
```

The borrow checker guarantees that you can't read from the buffer while writing to it — data races are impossible at compile time.

## What You Get for Free

- **No use-after-free bugs** — the compiler tracks lifetimes
- **No data races** — the ownership system prevents concurrent mutation
- **No null pointer dereferences** — `Option<T>` replaces null
- **No buffer overflows** — bounds checking is guaranteed

## The Trade-off

You spend more time fighting the borrow checker initially. But every compilation error is a bug that won't happen at runtime. After a few months, the borrow checker becomes your ally, not your enemy.

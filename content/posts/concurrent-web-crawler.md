# Building a Concurrent Web Crawler in Rust

Design and implementation of a high-performance web crawler using Tokio, Reqwest, and async Rust patterns.

## Architecture Overview

A web crawler has three core components:

1. **Frontier** — URLs to visit, prioritized and deduplicated
2. **Fetcher** — Downloads pages concurrently
3. **Parser** — Extracts links and content

```rust
use std::collections::{HashSet, VecDeque};
use std::sync::Arc;
use tokio::sync::Mutex;

struct Crawler {
    visited: Arc<Mutex<HashSet<String>>>,
    queue: Arc<Mutex<VecDeque<String>>>,
    max_concurrency: usize,
}
```

## Concurrent Fetching with Tokio

Tokio's task-per-request model works well for I/O-bound crawlers:

```rust
use reqwest::Client;
use tokio::sync::Semaphore;

async fn fetch_all(urls: Vec<String>, max_concurrent: usize) {
    let semaphore = Arc::new(Semaphore::new(max_concurrent));
    let client = Client::new();

    let mut handles = Vec::new();
    for url in urls {
        let sem = semaphore.clone();
        let client = client.clone();
        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            match client.get(&url).send().await {
                Ok(resp) => {
                    let body = resp.text().await.unwrap_or_default();
                    println!("Fetched {} ({} bytes)", url, body.len());
                }
                Err(e) => eprintln!("Failed {}: {}", url, e),
            }
        }));
    }

    for handle in handles {
        let _ = handle.await;
    }
}
```

## URL Deduplication

Use a Bloom filter for memory-efficient URL deduplication with millions of URLs:

```rust
use bloom::BloomFilter;

let mut filter = BloomFilter::with_rate(0.01, 1_000_000);
filter.insert("https://example.com");
assert!(filter.contains("https://example.com"));
```

## Politeness: Rate Limiting

Always respect `robots.txt` and add delays between requests to the same domain:

```rust
use std::time::Duration;

tokio::time::sleep(Duration::from_millis(200)).await;
```

## Handling Redirects and Errors

```rust
let response = client
    .get(&url)
    .redirect(reqwest::redirect::Policy::limited(5))
    .timeout(Duration::from_secs(10))
    .send()
    .await?;

match response.status().as_u16() {
    200 => Ok(response.text().await?),
    301 | 302 => {
        let location = response.headers().get("location")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();
        Ok(location)
    }
    429 => {
        tokio::time::sleep(Duration::from_secs(5)).await;
        Err(CrawlError::RateLimited)
    }
    code => Err(CrawlError::UnexpectedStatus(code)),
}
```

## Conclusion

Building a crawler in Rust gives you memory safety, fearless concurrency, and near-zero overhead. Tokio's async runtime handles thousands of concurrent connections efficiently, and Rust's type system catches bugs at compile time that would be runtime disasters in other languages.

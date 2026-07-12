# Error Handling Patterns in Rust

From Result and Option to custom error types and the thiserror/anyhow ecosystem. Battle-tested patterns for production code.

## The Two Pillars: Result and Option

Rust doesn't have exceptions. Instead, it uses `Result<T, E>` for fallible operations and `Option<T>` for nullable values:

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn find_user(id: u64) -> Option<User> {
    users.iter().find(|u| u.id == id).cloned()
}
```

## The ? Operator

The `?` operator propagates errors up the call stack elegantly:

```rust
fn read_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;  // propagates io::Error
    let config: Config = serde_json::from_str(&content)?;  // propagates json error
    Ok(config)
}
```

## Custom Error Types with thiserror

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found: {resource}")]
    NotFound { resource: String },

    #[error("Authentication failed")]
    Auth(#[from] AuthError),
}
```

`thiserror` auto-generates `Display` and `From` implementations, so you get clean error conversion.

## Error Context with anyhow

For application code where you don't need custom error types, `anyhow` provides ergonomic error handling:

```rust
use anyhow::{Context, Result};

fn process_file(path: &str) -> Result<String> {
    let content = std::fs::read_to_string(path)
        .context(format!("Failed to read {}", path))?;

    let parsed: Data = serde_json::from_str(&content)
        .context("Failed to parse JSON")?;

    Ok(format!("Processed {} items", parsed.items.len()))
}
```

## Patterns for Web Handlers

```rust
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

async fn get_post(State(db): State<Db>, Path(id): Path<u64>) -> Result\Json<Post>, AppError> {
    let post = db.get_post(id).await?
        .ok_or_else(|| AppError::NotFound {
            resource: format!("Post {}", id),
        })?;
    Ok(Json(post))
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound { .. } => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Auth(_) => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".into()),
        };
        (status, message).into_response()
    }
}
```

## When to Use What

- **`Option<T>`** — when absence is expected and not an error
- **`Result<T, E>`** — when you need to know why something failed
- **`anyhow`** — for application-level code, rapid prototyping
- **`thiserror`** — for library code where callers need to match on error variants

## Conclusion

Rust's error handling forces you to think about failure modes upfront. This feels verbose initially, but it produces robust, maintainable code where error paths are explicit and well-tested.

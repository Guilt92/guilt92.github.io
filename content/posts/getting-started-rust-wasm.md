# Getting Started with Rust WebAssembly

A practical guide to building high-performance web applications with Rust, WebAssembly, and Leptos.

## Why Rust + WASM?

WebAssembly gives you near-native performance in the browser. Rust's zero-cost abstractions and memory safety make it the ideal language for compiling to WASM — no garbage collector overhead, no runtime surprises.

```rust
use leptos::prelude::*;

#[component]
fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0);
    view! {
        <button on:click=move |_| set_count.update(|n| *n += 1)>
            "Clicks: " {count}
        </button>
    }
}
```

## Setting Up Your Environment

You'll need the WASM target and Trunk for building:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### Project Configuration

In your `Cargo.toml`, enable the CSR feature:

```toml
[dependencies]
leptos = { version = "0.8", features = ["csr"] }
wasm-bindgen = "0.2"
```

## Building Your First Component

Leptos components are regular Rust functions with the `#[component]` attribute. They return `impl IntoView`, which is Leptos's way of describing what should be rendered.

```rust
#[component]
fn BlogPost(title: String, content: String) -> impl IntoView {
    view! {
        <article>
            <h2>{title}</h2>
            <div inner_html=content />
        </article>
    }
}
```

### Reactive Signals

Signals are the backbone of Leptos reactivity. They're `Copy` types that can be moved into closures without cloning:

```rust
let (search_query, set_search_query) = signal(String::new());
let filtered = move || {
    posts.iter()
        .filter(|p| p.title.contains(&search_query.get()))
        .collect::<Vec<_>>()
};
```

## Deploying to GitHub Pages

Trunk builds your WASM app into static files. GitHub Pages serves them directly:

```yaml
# .github/workflows/deploy.yml
- name: Install WASM tools
  run: |
    rustup target add wasm32-unknown-unknown
    cargo install trunk

- name: Build
  run: trunk build --release

- name: Deploy
  uses: actions/deploy-pages@v4
  with:
    publish_dir: ./dist
```

## Key Takeaways

- Rust + WASM gives you near-native performance in the browser
- Leptos provides a React-like component model with fine-grained reactivity
- Trunk handles the build toolchain, so you focus on writing Rust
- GitHub Pages makes deployment trivial with GitHub Actions

The entire blog you're reading was built this way — Rust compiled to WASM, served as static files.

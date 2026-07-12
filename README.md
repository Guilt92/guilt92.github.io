# DevNotes

A technical blog built with Rust, Leptos, and WebAssembly. Deployed to GitHub Pages via Trunk.

## Features

- **Rust/WASM**: Full application logic in Rust, compiled to WebAssembly
- **Leptos CSR**: Client-side rendered SPA with fine-grained reactivity
- **Custom Markdown Renderer**: Pure Rust markdown-to-HTML with syntax highlighting
- **Dark/Light Mode**: Automatic theme switching with system preference detection
- **Client-Side Search**: Real-time search across posts by title, tags, and category
- **Responsive Design**: Mobile-first layout with collapsible sidebar
- **Hash-Based Routing**: SPA navigation without page reloads
- **Scroll-to-Top**: Animated scroll button

## Tech Stack

| Component | Technology |
|-----------|-----------|
| Language | Rust |
| Framework | Leptos 0.8 (CSR) |
| Build Tool | Trunk 0.21 |
| WASM Target | wasm32-unknown-unknown |
| Styling | Custom CSS (no framework) |
| Icons | Font Awesome 6 |
| Fonts | Inter + JetBrains Mono |
| Hosting | GitHub Pages |

## Development

### Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### Local Development

```bash
trunk serve
```

Opens at `http://localhost:8080`.

### Production Build

```bash
trunk build --release
```

Output goes to `dist/`.

## Project Structure

```
guilt92.github.io/
├── src/
│   └── lib.rs              # Leptos app, components, markdown renderer
├── content/
│   └── posts/              # Markdown blog post files
├── style.css               # Complete CSS design system
├── index.html              # Trunk entry point
├── Cargo.toml              # Rust dependencies
├── Trunk.toml              # Trunk build configuration
├── .github/
│   └── workflows/
│       └── deploy.yml      # GitHub Actions CI/CD
└── README.md
```

## Adding a New Post

1. Create a markdown file in `content/posts/your-post-slug.md`
2. Add a new `BlogPost` entry in `src/lib.rs` in the `all_posts()` function
3. Use `include_str!("../content/posts/your-post-slug.md")` for the content field
4. Push to `main` — GitHub Actions auto-deploys

## Deployment

Every push to `main` triggers the GitHub Actions workflow which:
1. Installs Rust + WASM target + Trunk
2. Builds with `trunk build --release`
3. Deploys `dist/` to GitHub Pages

## License

MIT

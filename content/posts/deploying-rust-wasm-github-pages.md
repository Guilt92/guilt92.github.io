# Deploying Rust WASM to GitHub Pages

Complete CI/CD pipeline for building, optimizing, and deploying a Leptos WASM application to GitHub Pages with Trunk.

## Prerequisites

- Rust toolchain with `wasm32-unknown-unknown` target
- Trunk installed (`cargo install trunk`)
- A GitHub repository with Pages enabled

## Project Setup

### Trunk Configuration

Create `Trunk.toml` in your project root:

```toml
[build]
target = "index.html"
dist = "dist"

[watch]
watch = ["src", "index.html", "style.css"]

[serve]
address = "127.0.0.1"
port = 8080
```

### Entry Point

Your `index.html` needs the Trunk-specific rust link:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>My WASM App</title>
    <link data-trunk rel="rust" data-wasm-opt="2" />
</head>
<body>
</body>
</html>
```

The `data-trunk rel="rust"` attribute tells Trunk to compile your Rust code and inject the WASM loader.

## Building Locally

```bash
# Development build
trunk serve

# Production build
trunk build --release
```

The `--release` flag enables full optimizations. The output goes to `dist/`.

## GitHub Actions Workflow

```yaml
name: Deploy to GitHub Pages

on:
  push:
    branches: [main]

permissions:
  contents: read
  pages: write
  id-token: write

concurrency:
  group: "pages"
  cancel-in-progress: false

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Install Trunk
        uses: taiki-e/install-action@trunk

      - name: Cache Rust dependencies
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Build
        run: trunk build --release

      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: ./dist

  deploy:
    needs: build
    runs-on: ubuntu-latest
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

## Optimization Tips

### WASM Size Reduction

```toml
# In Cargo.toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Single codegen unit for better optimization
strip = true        # Strip debug symbols
panic = "abort"     # Use abort instead of unwind
```

### Additional Tools

```bash
# Install wasm-opt for further optimization
cargo install wasm-opt

# Analyze binary size
cargo install cargo-bloat
cargo bloat --release -n 30

# Or use twiggy
cargo install twiggy
twiggy top dist/*.wasm
```

## Common Issues

### WASM Too Large

If your WASM exceeds 5MB, check:
- Are you pulling in heavy dependencies?
- Is `opt-level = "z"` set in your release profile?
- Can you lazy-load some components?

### GitHub Pages 404

Ensure:
- Pages source is set to `gh-pages` branch or `dist/` directory
- The `baseurl` in your config matches your repository name
- Your workflow has the correct permissions

## Conclusion

The pipeline is straightforward: Rust → WASM → Static files → GitHub Pages. Trunk handles the complexity of the build, and GitHub Actions automates deployment on every push.

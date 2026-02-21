# Dev Notes (scaffold)

This repository now contains a minimal, developer-focused static blog scaffold inspired by modern technical blogs.

Features:

- Blog listing with categories and tags
- Client-side search
- Markdown posts in `content/posts/`
- Syntax highlighting with Prism
- Responsive, minimal UI

To run locally (simple static server):

```sh
python3 -m http.server 8000
```

Then open http://localhost:8000

Add new posts by updating `content/posts.json` and adding markdown files under `content/posts/`.

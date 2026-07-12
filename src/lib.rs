use leptos::prelude::*;
use leptos::either::Either;
use wasm_bindgen::prelude::*;
use web_sys::window;

// ============================================
// Blog Post Data
// ============================================

#[derive(Clone, Debug, PartialEq)]
struct BlogPost {
    title: String,
    slug: String,
    date: String,
    excerpt: String,
    category: String,
    tags: Vec<String>,
    content: String,
}

fn all_posts() -> Vec<BlogPost> {
    vec![
        BlogPost {
            title: "Getting Started with Rust WebAssembly".into(),
            slug: "getting-started-rust-wasm".into(),
            date: "2025-12-15".into(),
            excerpt: "A practical guide to building high-performance web applications with Rust, WebAssembly, and Leptos. From zero to deployed.".into(),
            category: "Rust".into(),
            tags: vec!["rust".into(), "wasm".into(), "web".into(), "leptos".into()],
            content: include_str!("../content/posts/getting-started-rust-wasm.md").into(),
        },
        BlogPost {
            title: "Zero-Cost Abstractions in Practice".into(),
            slug: "zero-cost-abstractions".into(),
            date: "2025-11-28".into(),
            excerpt: "Understanding what \"zero-cost\" really means in Rust, with assembly analysis and benchmarking techniques.".into(),
            category: "Rust".into(),
            tags: vec!["rust".into(), "performance".into(), "optimization".into()],
            content: include_str!("../content/posts/zero-cost-abstractions.md").into(),
        },
        BlogPost {
            title: "Building a Concurrent Web Crawler in Rust".into(),
            slug: "concurrent-web-crawler".into(),
            date: "2025-11-10".into(),
            excerpt: "Design and implementation of a high-performance web crawler using Tokio, Reqwest, and async Rust patterns.".into(),
            category: "Systems".into(),
            tags: vec!["rust".into(), "async".into(), "tokio".into(), "networking".into()],
            content: include_str!("../content/posts/concurrent-web-crawler.md").into(),
        },
        BlogPost {
            title: "Memory Safety Without Garbage Collection".into(),
            slug: "memory-safety-without-gc".into(),
            date: "2025-10-22".into(),
            excerpt: "Deep dive into Rust's ownership model, borrow checker, and how they eliminate entire classes of bugs at compile time.".into(),
            category: "Fundamentals".into(),
            tags: vec!["rust".into(), "memory".into(), "ownership".into(), "safety".into()],
            content: include_str!("../content/posts/memory-safety-without-gc.md").into(),
        },
        BlogPost {
            title: "Error Handling Patterns in Rust".into(),
            slug: "error-handling-patterns".into(),
            date: "2025-10-05".into(),
            excerpt: "From Result and Option to custom error types and the thiserror/anyhow ecosystem. Battle-tested patterns for production code.".into(),
            category: "Rust".into(),
            tags: vec!["rust".into(), "errors".into(), "patterns".into()],
            content: include_str!("../content/posts/error-handling-patterns.md").into(),
        },
        BlogPost {
            title: "Deploying Rust WASM to GitHub Pages".into(),
            slug: "deploying-rust-wasm-github-pages".into(),
            date: "2025-09-18".into(),
            excerpt: "Complete CI/CD pipeline for building, optimizing, and deploying a Leptos WASM application to GitHub Pages with Trunk.".into(),
            category: "DevOps".into(),
            tags: vec!["rust".into(), "wasm".into(), "ci-cd".into(), "github-actions".into()],
            content: include_str!("../content/posts/deploying-rust-wasm-github-pages.md").into(),
        },
        BlogPost {
            title: "Rust Traits and Generics Deep Dive".into(),
            slug: "traits-and-generics".into(),
            date: "2025-09-01".into(),
            excerpt: "Mastering trait objects, static dispatch, associated types, and advanced generic programming patterns in Rust.".into(),
            category: "Fundamentals".into(),
            tags: vec!["rust".into(), "traits".into(), "generics".into()],
            content: include_str!("../content/posts/traits-and-generics.md").into(),
        },
        BlogPost {
            title: "Async Rust: Futures, Tasks, and Executors".into(),
            slug: "async-rust-deep-dive".into(),
            date: "2025-08-15".into(),
            excerpt: "Understanding the async/await model in Rust from the ground up: how futures work, task scheduling, and the Tokio runtime.".into(),
            category: "Systems".into(),
            tags: vec!["rust".into(), "async".into(), "tokio".into(), "concurrency".into()],
            content: include_str!("../content/posts/async-rust-deep-dive.md").into(),
        },
    ]
}

// ============================================
// Pure Rust String Helpers
// ============================================

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

// ============================================
// Markdown to HTML Renderer
// ============================================

fn render_inline(text: &str) -> String {
    let mut result = text.to_string();

    // Inline code first (protect from other transforms)
    result = simple_regex_replace(&result, "`([^`]+)`", |caps| {
        format!("<code>{}</code>", &caps[1])
    });

    // Images
    result = simple_regex_replace(&result, r"!\[([^\]]*)\]\(([^)]+)\)", |caps| {
        format!("<img src=\"{}\" alt=\"{}\" loading=\"lazy\">", &caps[2], &caps[1])
    });

    // Links
    result = simple_regex_replace(&result, r"\[([^\]]+)\]\(([^)]+)\)", |caps| {
        format!("<a href=\"{}\" target=\"_blank\" rel=\"noopener\">{}</a>", &caps[2], &caps[1])
    });

    // Bold (**text**)
    result = simple_regex_replace(&result, r"\*\*([^*]+)\*\*", |caps| {
        format!("<strong>{}</strong>", &caps[1])
    });

    // Italic (*text*) — only single asterisks
    result = simple_regex_replace_italic(&result);

    result
}

/// Simple regex replacement with capture groups (no external regex crate needed)
fn simple_regex_replace(input: &str, _pattern: &str, f: impl Fn(Vec<&str>) -> String) -> String {
    // We implement a small subset: patterns with () groups
    // This is a simplified approach that handles common markdown patterns
    match _pattern {
        "`([^`]+)`" => {
            let mut out = String::new();
            let chars: Vec<char> = input.chars().collect();
            let len = chars.len();
            let mut i = 0;
            while i < len {
                if chars[i] == '`' {
                    if let Some(end) = chars[i+1..].iter().position(|&c| c == '`') {
                        let content: String = chars[i+1..i+1+end].iter().collect();
                        out.push_str(&f(vec![&content]));
                        i = i + 2 + end;
                        continue;
                    }
                }
                out.push(chars[i]);
                i += 1;
            }
            out
        }
        r"!\[([^\]]*)\]\(([^)]+)\)" => {
            let mut out = String::new();
            let s = input;
            let mut pos = 0;
            while let Some(bang_pos) = s[pos..].find("![") {
                let abs_bang = pos + bang_pos;
                out.push_str(&s[pos..abs_bang]);
                if let Some(close_bracket) = s[abs_bang+2..].find(']') {
                    let alt = &s[abs_bang+2..abs_bang+2+close_bracket];
                    let rest = &s[abs_bang+2+close_bracket+1..];
                    if rest.starts_with('(') {
                        if let Some(close_paren) = rest[1..].find(')') {
                            let url = &rest[1..1+close_paren];
                            out.push_str(&f(vec![alt, url]));
                            pos = abs_bang + 2 + close_bracket + 1 + 1 + close_paren + 1;
                            continue;
                        }
                    }
                    out.push_str(&s[abs_bang..abs_bang+2+close_bracket+1]);
                    pos = abs_bang + 2 + close_bracket + 1;
                } else {
                    out.push_str(&s[abs_bang..]);
                    pos = s.len();
                }
            }
            out.push_str(&s[pos..]);
            out
        }
        r"\[([^\]]+)\]\(([^)]+)\)" => {
            let mut out = String::new();
            let s = input;
            let mut pos = 0;
            while let Some(bracket_pos) = s[pos..].find('[') {
                let abs_bracket = pos + bracket_pos;
                // Make sure it's not preceded by '!'
                if abs_bracket > 0 && s.as_bytes()[abs_bracket - 1] == b'!' {
                    out.push_str(&s[pos..abs_bracket+1]);
                    pos = abs_bracket + 1;
                    continue;
                }
                out.push_str(&s[pos..abs_bracket]);
                if let Some(close_bracket) = s[abs_bracket+1..].find(']') {
                    let text = &s[abs_bracket+1..abs_bracket+1+close_bracket];
                    let rest = &s[abs_bracket+1+close_bracket+1..];
                    if rest.starts_with('(') {
                        if let Some(close_paren) = rest[1..].find(')') {
                            let url = &rest[1..1+close_paren];
                            out.push_str(&f(vec![text, url]));
                            pos = abs_bracket + 1 + close_bracket + 1 + 1 + close_paren + 1;
                            continue;
                        }
                    }
                    out.push_str(&s[abs_bracket..abs_bracket+1+close_bracket+1]);
                    pos = abs_bracket + 1 + close_bracket + 1;
                } else {
                    out.push_str(&s[abs_bracket..]);
                    pos = s.len();
                }
            }
            out.push_str(&s[pos..]);
            out
        }
        r"\*\*([^*]+)\*\*" => {
            let mut out = String::new();
            let s = input;
            let mut pos = 0;
            while let Some(star_pos) = s[pos..].find("**") {
                let abs_star = pos + star_pos;
                out.push_str(&s[pos..abs_star]);
                if let Some(close_star) = s[abs_star+2..].find("**") {
                    let content = &s[abs_star+2..abs_star+2+close_star];
                    out.push_str(&f(vec![content]));
                    pos = abs_star + 2 + close_star + 2;
                } else {
                    out.push_str("**");
                    pos = abs_star + 2;
                }
            }
            out.push_str(&s[pos..]);
            out
        }
        _ => input.to_string(),
    }
}

fn simple_regex_replace_italic(input: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if chars[i] == '*' && (i == 0 || chars[i-1] != '*') {
            // Look for closing single asterisk (not double)
            if let Some(star_end) = chars[i+1..].iter().position(|&c| c == '*') {
                let abs_end = i + 1 + star_end;
                // Check it's not a double asterisk
                if abs_end + 1 < len && chars[abs_end + 1] == '*' {
                    // It's part of **, skip
                    out.push(chars[i]);
                    i += 1;
                    continue;
                }
                // Check the char before closing * is not *
                if star_end > 0 && chars[i + star_end] == '*' {
                    out.push(chars[i]);
                    i += 1;
                    continue;
                }
                let content: String = chars[i+1..abs_end].iter().collect();
                if !content.is_empty() {
                    out.push_str(&format!("<em>{}</em>", content));
                    i = abs_end + 1;
                    continue;
                }
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

fn render_markdown_with_highlighting(md: &str) -> String {
    let mut html = String::new();
    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_buffer = String::new();
    let lines: Vec<&str> = md.lines().collect();
    let mut line_idx = 0;

    while line_idx < lines.len() {
        let line = lines[line_idx];
        line_idx += 1;

        if line.starts_with("```") {
            if in_code_block {
                let highlighted = highlight_code(&code_buffer, &code_lang);
                html.push_str(&format!(
                    "<div class=\"code-block\"><div class=\"code-header\"><span class=\"code-lang\">{}</span></div><pre>{}</pre></div>\n",
                    code_lang, highlighted
                ));
                code_buffer.clear();
                in_code_block = false;
            } else {
                in_code_block = true;
                code_lang = line.trim_start_matches('`').trim().to_string();
            }
            continue;
        }

        if in_code_block {
            if !code_buffer.is_empty() {
                code_buffer.push('\n');
            }
            code_buffer.push_str(line);
            continue;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("#### ") {
            html.push_str(&format!("<h4>{}</h4>\n", render_inline(rest)));
        } else if let Some(rest) = trimmed.strip_prefix("### ") {
            html.push_str(&format!("<h3>{}</h3>\n", render_inline(rest)));
        } else if let Some(rest) = trimmed.strip_prefix("## ") {
            html.push_str(&format!("<h2>{}</h2>\n", render_inline(rest)));
        } else if let Some(rest) = trimmed.strip_prefix("# ") {
            html.push_str(&format!("<h1>{}</h1>\n", render_inline(rest)));
        } else if trimmed == "---" || trimmed == "***" {
            html.push_str("<hr>\n");
        } else if let Some(rest) = trimmed.strip_prefix("> ") {
            html.push_str(&format!("<blockquote>{}</blockquote>\n", render_inline(rest)));
        } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            html.push_str("<ul>\n");
            let prefix_len = if trimmed.starts_with("- ") { 2 } else { 2 };
            let first = &trimmed[prefix_len..];
            html.push_str(&format!("<li>{}</li>\n", render_inline(first)));
            while line_idx < lines.len() {
                let next = lines[line_idx].trim();
                if next.starts_with("- ") || next.starts_with("* ") {
                    html.push_str(&format!("<li>{}</li>\n", render_inline(&next[2..])));
                    line_idx += 1;
                } else {
                    break;
                }
            }
            html.push_str("</ul>\n");
        } else {
            html.push_str(&format!("<p>{}</p>\n", render_inline(trimmed)));
        }
    }

    if in_code_block && !code_buffer.is_empty() {
        let highlighted = highlight_code(&code_buffer, &code_lang);
        html.push_str(&format!(
            "<div class=\"code-block\"><div class=\"code-header\"><span class=\"code-lang\">{}</span></div><pre>{}</pre></div>\n",
            code_lang, highlighted
        ));
    }

    html
}

// ============================================
// Basic Syntax Highlighting
// ============================================

fn highlight_code(code: &str, lang: &str) -> String {
    let escaped = escape_html(code);
    match lang {
        "rust" | "rs" => highlight_keywords(&escaped, &[
            "fn", "let", "mut", "pub", "use", "mod", "struct", "enum", "impl", "trait",
            "where", "match", "if", "else", "for", "while", "loop", "return", "async",
            "await", "move", "self", "Self", "super", "crate", "const", "static", "type",
            "ref", "as", "in", "dyn", "unsafe", "extern", "true", "false",
        ]),
        "toml" => highlight_keywords(&escaped, &["true", "false"]),
        "bash" | "sh" | "shell" => highlight_keywords(&escaped, &[
            "echo", "cd", "ls", "mkdir", "cargo", "git", "npm", "curl", "sudo",
            "rustup", "trunk", "cat", "grep", "sed", "awk",
        ]),
        "json" => highlight_keywords(&escaped, &["true", "false", "null"]),
        "yaml" | "yml" => highlight_keywords(&escaped, &["true", "false", "null"]),
        "html" | "css" => highlight_keywords(&escaped, &[]),
        _ => format!("<code>{}</code>", escaped),
    }
}

fn highlight_keywords(code: &str, keywords: &[&str]) -> String {
    let mut result = code.to_string();

    // Highlight keywords
    for kw in keywords {
        let word_boundary = format!(" {} ", kw);
        let replacement = format!(" <span class=\"token-keyword\">{}</span> ", kw);
        result = result.replace(&word_boundary, &replacement);
    }

    // Highlight strings
    let mut new_result = String::new();
    let chars: Vec<char> = result.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if chars[i] == '"' {
            new_result.push_str("<span class=\"token-string\">\"");
            i += 1;
            while i < len && chars[i] != '"' {
                if chars[i] == '\\' && i + 1 < len {
                    new_result.push(chars[i]);
                    new_result.push(chars[i + 1]);
                    i += 2;
                } else {
                    new_result.push(chars[i]);
                    i += 1;
                }
            }
            if i < len {
                new_result.push_str("\"</span>");
                i += 1;
            }
        } else if chars[i] == '/' && i + 1 < len && chars[i + 1] == '/' {
            new_result.push_str("<span class=\"token-comment\">");
            while i < len && chars[i] != '\n' {
                new_result.push(chars[i]);
                i += 1;
            }
            new_result.push_str("</span>");
        } else {
            new_result.push(chars[i]);
            i += 1;
        }
    }

    result = new_result;

    // Highlight numbers
    let mut num_result = String::new();
    let chars: Vec<char> = result.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if chars[i].is_ascii_digit() && (i == 0 || !chars[i-1].is_alphanumeric()) {
            num_result.push_str("<span class=\"token-number\">");
            while i < len && (chars[i].is_ascii_digit() || chars[i] == '_') {
                num_result.push(chars[i]);
                i += 1;
            }
            num_result.push_str("</span>");
        } else {
            num_result.push(chars[i]);
            i += 1;
        }
    }

    num_result
}

// ============================================
// App Component
// ============================================

#[component]
fn App() -> impl IntoView {
    let (sidebar_open, set_sidebar_open) = signal(false);
    let (dark_mode, set_dark_mode) = signal(true);

    // Initialize dark mode from localStorage / system preference
    let init_dark = move || {
        if let Some(win) = window() {
            if let Ok(storage) = win.local_storage() {
                if let Some(storage) = storage {
                    if let Ok(Some(mode)) = storage.get_item("theme") {
                        return mode == "dark";
                    }
                }
            }
            if let Ok(Some(mq)) = win.match_media("(prefers-color-scheme: dark)") {
                return mq.matches();
            }
        }
        true
    };

    let initial = init_dark();
    set_dark_mode.set(initial);

    let apply_theme = move |is_dark: bool| {
        if let Some(win) = window() {
            if let Some(doc) = win.document() {
                if let Some(html) = doc.document_element() {
                    let _ = html.set_attribute(
                        "data-theme",
                        if is_dark { "dark" } else { "light" },
                    );
                }
                if let Ok(Some(storage)) = win.local_storage() {
                    let _ = storage.set_item(
                        "theme",
                        if is_dark { "dark" } else { "light" },
                    );
                }
            }
        }
    };

    apply_theme(initial);

    let parse_hash = move || -> String {
        let hash = window()
            .and_then(|w| w.location().hash().ok())
            .unwrap_or_default();
        let h = hash.trim_start_matches('#').trim_start_matches('/');
        if h.is_empty() { "/".to_string() } else { format!("/{}", h) }
    };

    let (route_read, route_write) = signal(parse_hash());
    let set_route = route_write;

    // Keep the interval alive for the component lifetime
    let _route_interval;
    {
        let set_route = set_route;
        let route_read = route_read;
        _route_interval = gloo_timers::callback::Interval::new(100, move || {
            let new_route = parse_hash();
            let current = route_read.get_untracked();
            if new_route != current {
                set_route.set(new_route);
            }
        });
    }

    let toggle_theme = move |_| {
        let new_val = !dark_mode.get();
        set_dark_mode.set(new_val);
        apply_theme(new_val);
    };

    let toggle_sidebar = move |_| {
        set_sidebar_open.update(|v| *v = !*v);
    };

    let close_sidebar = move |_| {
        set_sidebar_open.set(false);
    };

    let posts = all_posts();

    view! {
        <div class="app-layout">
            <div
                class=move || {
                    let open = sidebar_open.get();
                    if open { "sidebar-overlay active" } else { "sidebar-overlay" }
                }
                on:click=close_sidebar
            />

            <aside class=move || {
                let open = sidebar_open.get();
                if open { "sidebar open" } else { "sidebar" }
            }>
                <div class="sidebar-header">
                    <div class="avatar">"DN"</div>
                    <div class="site-title">"DevNotes"</div>
                    <div class="site-subtitle">"Systems programming, demystified."</div>
                </div>
                <nav class="sidebar-nav">
                    <div class="nav-item">
                        <a class=move || {
                            let r = route_read.get();
                            if r == "/" || r.is_empty() { "nav-link active" } else { "nav-link" }
                        }
                        href="#/"
                        on:click=close_sidebar>
                            <i class="fas fa-home"></i>
                            <span>"HOME"</span>
                        </a>
                    </div>
                    <div class="nav-item">
                        <a class=move || {
                            let r = route_read.get();
                            if r == "/about" { "nav-link active" } else { "nav-link" }
                        }
                        href="#/about"
                        on:click=close_sidebar>
                            <i class="fas fa-info-circle"></i>
                            <span>"ABOUT"</span>
                        </a>
                    </div>
                </nav>
                <div class="sidebar-footer">
                    <a href="https://github.com/guilt92" target="_blank" rel="noopener" class="social-link" aria-label="GitHub">
                        <i class="fab fa-github"></i>
                    </a>
                    <a href="https://twitter.com/guilt92" target="_blank" rel="noopener" class="social-link" aria-label="Twitter">
                        <i class="fab fa-x-twitter"></i>
                    </a>
                    <a href="mailto:guilt92@users.noreply.github.com" class="social-link" aria-label="Email">
                        <i class="fas fa-envelope"></i>
                    </a>
                </div>
            </aside>

            <div class="main-wrapper">
                <header class="topbar">
                    <div class="topbar-left">
                        <button class="sidebar-toggle" on:click=toggle_sidebar aria-label="Toggle sidebar">
                            <i class="fas fa-bars"></i>
                        </button>
                        <nav class="breadcrumb">
                            <span>"Home"</span>
                        </nav>
                    </div>
                    <div class="topbar-title">"DevNotes"</div>
                    <div class="topbar-right">
                        <SearchBox posts=posts.clone() />
                        <button class="mode-toggle" on:click=toggle_theme aria-label="Toggle theme">
                            {move || {
                                if dark_mode.get() {
                                    view! { <i class="fas fa-sun"></i> }.into_view()
                                } else {
                                    view! { <i class="fas fa-moon"></i> }.into_view()
                                }
                            }}
                        </button>
                    </div>
                </header>

                <div class="content-area">
                    <div class="content-inner">
                        <main class="main-col">
                            {{
                                let posts_clone = posts.clone();
                                move || {
                                    let r = route_read.get();
                                    let p = posts_clone.clone();
                                    if r == "/about" {
                                        Either::Left(view! { <AboutPage /> })
                                    } else if r.starts_with("/post/") {
                                        let slug = r.trim_start_matches("/post/").to_string();
                                        Either::Right(Either::Left(view! { <PostPage slug=slug posts=p /> }))
                                    } else {
                                        Either::Right(Either::Right(view! { <HomePage posts=p /> }))
                                    }
                                }
                            }}
                        </main>
                        <aside class="side-panel">
                            <SidePanel posts=posts.clone() />
                        </aside>
                    </div>
                    <footer class="site-footer">
                        <p>"© 2025 DevNotes · Built with "<a href="https://github.com/leptos-rs/leptos" target="_blank">"Leptos"</a>" + Rust + WebAssembly"</p>
                    </footer>
                </div>
            </div>

            <ScrollTop />
        </div>
    }
}

// ============================================
// Search Box
// ============================================

#[component]
fn SearchBox(posts: Vec<BlogPost>) -> impl IntoView {
    let (query, set_query) = signal(String::new());
    let (show_results, set_show_results) = signal(false);

    let on_input = move |ev: web_sys::Event| {
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                let val = input.value();
                set_query.set(val.clone());
                set_show_results.set(!val.is_empty());
            }
        }
    };

    let on_focus = move |_| {
        if !query.get().is_empty() {
            set_show_results.set(true);
        }
    };

    let on_blur = move |_| {
        let _timeout = gloo_timers::callback::Timeout::new(200, move || {
            set_show_results.set(false);
        });
    };

    view! {
        <div class="search-wrapper">
            <i class="fas fa-search search-icon"></i>
            <input
                class="search-input"
                type="search"
                placeholder="Search posts..."
                aria-label="Search posts"
                prop:value=move || query.get()
                on:input=on_input
                on:focus=on_focus
                on:blur=on_blur
            />
            <div class=move || {
                let show = show_results.get();
                if show { "search-results active" } else { "search-results" }
            }>
                {move || {
                    let q = query.get().to_lowercase();
                    let results: Vec<BlogPost> = if q.is_empty() {
                        vec![]
                    } else {
                        posts.iter().filter(|p| {
                            p.title.to_lowercase().contains(&q)
                                || p.excerpt.to_lowercase().contains(&q)
                                || p.tags.iter().any(|t| t.to_lowercase().contains(&q))
                                || p.category.to_lowercase().contains(&q)
                        }).cloned().collect()
                    };
                    let is_empty_and_queried = results.is_empty() && !q.is_empty();
                    view! {
                        <div>
                            {if is_empty_and_queried {
                                Either::Left::<_, leptos::prelude::View<_>>(view! { <div class="search-no-results">"No results found."</div> })
                            } else {
                                Either::Right(view! {
                                    <div>
                                        {results.into_iter().map(|p| {
                                            let slug = p.slug.clone();
                                            let title = p.title.clone();
                                            let excerpt = p.excerpt.clone();
                                            view! {
                                                <div class="search-result-item"
                                                    on:click=move |_| {
                                                        if let Some(win) = window() {
                                                            let _ = win.location().set_hash(&format!("/post/{}", slug));
                                                        }
                                                        set_query.set(String::new());
                                                        set_show_results.set(false);
                                                    }
                                                >
                                                    <div class="search-result-title">{title}</div>
                                                    <div class="search-result-excerpt">{excerpt}</div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                })
                            }}
                        </div>
                    }
                }}
            </div>
        </div>
    }
}

// ============================================
// Home Page
// ============================================

#[component]
fn HomePage(posts: Vec<BlogPost>) -> impl IntoView {
    let mut sorted = posts.clone();
    sorted.sort_by(|a, b| b.date.cmp(&a.date));

    let featured = sorted.first().cloned();

    view! {
        <div>
            <section class="hero">
                <div class="hero-content">
                    <div class="hero-text">
                        <h1>"Concise, practical developer notes"</h1>
                        <p>"Deep-dives, tooling tips, and clean code examples for systems engineers."</p>
                    </div>
                    {featured.map(|p| {
                        let slug = p.slug.clone();
                        let title = p.title.clone();
                        let category = p.category.clone();
                        let date = p.date.clone();
                        let excerpt = p.excerpt.clone();
                        view! {
                            <a class="hero-card" href=format!("#/post/{}", slug)>
                                <div class="post-card-meta">
                                    <span class="post-card-category">{category}</span>
                                    <span>{date}</span>
                                </div>
                                <div class="post-card-title">{title}</div>
                                <div class="post-card-excerpt">{excerpt}</div>
                            </a>
                        }
                    })}
                </div>
            </section>

            <section class="post-list">
                {sorted.into_iter().enumerate().map(|(i, post)| {
                    let delay = match i % 3 {
                        0 => "animate-in-delay-1",
                        1 => "animate-in-delay-2",
                        _ => "animate-in-delay-3",
                    };
                    let slug = post.slug.clone();
                    let title = post.title.clone();
                    let excerpt = post.excerpt.clone();
                    let date = post.date.clone();
                    let category = post.category.clone();
                    let tags = post.tags.clone();
                    let class_str = format!("post-card animate-in {}", delay);
                    view! {
                        <article class=class_str>
                            <a href=format!("#/post/{}", slug) style="text-decoration:none;color:inherit;">
                                <div class="post-card-meta">
                                    <span class="post-card-category">{category}</span>
                                    <span><i class="far fa-calendar"></i>{date}</span>
                                </div>
                                <h2 class="post-card-title">{title}</h2>
                                <p class="post-card-excerpt">{excerpt}</p>
                                <div class="post-card-tags">
                                    {tags.into_iter().map(|t| {
                                        view! { <span class="tag">{t}</span> }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </a>
                        </article>
                    }
                }).collect::<Vec<_>>()}
            </section>
        </div>
    }
}

// ============================================
// Post Page
// ============================================

#[component]
fn PostPage(slug: String, posts: Vec<BlogPost>) -> impl IntoView {
    let post = posts.iter().find(|p| p.slug == slug).cloned();

    match post {
        None => Either::Left::<_, leptos::prelude::View<_>>(view! {
            <div class="empty-state">
                <i class="fas fa-file-alt"></i>
                <p>"Post not found."</p>
                <a href="#/" class="back-link" style="justify-content:center;margin-top:16px;">
                    <i class="fas fa-arrow-left"></i>
                    "Back to home"
                </a>
            </div>
        }),
        Some(p) => {
            let html_content = render_markdown_with_highlighting(&p.content);
            Either::Right(view! {
                <div class="post-page animate-in">
                    <a href="#/" class="back-link">
                        <i class="fas fa-arrow-left"></i>
                        "Back to all posts"
                    </a>
                    <article>
                        <header class="post-header">
                            <h1 class="post-title">{p.title}</h1>
                            <div class="post-meta">
                                <span><i class="far fa-calendar"></i>{p.date}</span>
                                <span><i class="far fa-folder"></i>{p.category}</span>
                                <span><i class="fas fa-tags"></i>{p.tags.join(", ")}</span>
                            </div>
                        </header>
                        <div class="post-content" inner_html=html_content />
                    </article>
                </div>
            })
        }
    }
}

// ============================================
// About Page
// ============================================

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <div class="about-page animate-in">
            <h1>"About DevNotes"</h1>
            <div class="about-card">
                <p>
                    "A technical blog focused on deep-dive analysis of systems programming, distributed systems, "
                    "software architecture, and developer tooling. Built entirely in Rust and compiled to WebAssembly."
                </p>
            </div>

            <h2>"What I Write About"</h2>
            <ul style="padding-left:20px;list-style:disc;">
                <li style="margin-bottom:8px;color:var(--text-secondary);">"Rust language internals and advanced patterns"</li>
                <li style="margin-bottom:8px;color:var(--text-secondary);">"WebAssembly and browser-level systems programming"</li>
                <li style="margin-bottom:8px;color:var(--text-secondary);">"Concurrent and async programming architectures"</li>
                <li style="margin-bottom:8px;color:var(--text-secondary);">"Performance engineering and optimization"</li>
                <li style="margin-bottom:8px;color:var(--text-secondary);">"Developer tooling and workflow design"</li>
            </ul>

            <h2>"Tech Stack"</h2>
            <div class="about-card">
                <p style="margin-bottom:8px;">"This site is built with:"</p>
                <ul style="padding-left:20px;list-style:disc;">
                    <li style="margin-bottom:6px;color:var(--text-secondary);">"Rust + Leptos (client-side rendered SPA)"</li>
                    <li style="margin-bottom:6px;color:var(--text-secondary);">"WebAssembly (compiled via Trunk)"</li>
                    <li style="margin-bottom:6px;color:var(--text-secondary);">"Custom markdown renderer with syntax highlighting"</li>
                    <li style="margin-bottom:6px;color:var(--text-secondary);">"GitHub Actions CI/CD for automated deployment"</li>
                </ul>
            </div>

            <h2>"Connect"</h2>
            <div class="about-links">
                <a href="https://github.com/guilt92" target="_blank" rel="noopener" class="about-link">
                    <i class="fab fa-github"></i>
                    "GitHub"
                </a>
                <a href="https://twitter.com/guilt92" target="_blank" rel="noopener" class="about-link">
                    <i class="fab fa-x-twitter"></i>
                    "Twitter / X"
                </a>
                <a href="mailto:guilt92@users.noreply.github.com" class="about-link">
                    <i class="fas fa-envelope"></i>
                    "Email"
                </a>
            </div>
        </div>
    }
}

// ============================================
// Side Panel
// ============================================

#[component]
fn SidePanel(posts: Vec<BlogPost>) -> impl IntoView {
    let mut sorted = posts.clone();
    sorted.sort_by(|a, b| b.date.cmp(&a.date));

    let all_tags: Vec<String> = {
        let mut tags: Vec<String> = posts.iter().flat_map(|p| p.tags.clone()).collect();
        tags.sort();
        tags.dedup();
        tags
    };

    let all_categories: Vec<String> = {
        let mut cats: Vec<String> = posts.iter().map(|p| p.category.clone()).collect();
        cats.sort();
        cats.dedup();
        cats
    };

    view! {
        <section class="panel-section">
            <h2 class="panel-heading">"Recently Updated"</h2>
            <div class="panel-list">
                {sorted.into_iter().take(5).map(|p| {
                    let slug = p.slug.clone();
                    let title = p.title.clone();
                    view! {
                        <div class="panel-list-item"
                            on:click=move |_| {
                                if let Some(win) = window() {
                                    let _ = win.location().set_hash(&format!("/post/{}", slug));
                                }
                            }
                        >
                            {title}
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </section>

        <section class="panel-section">
            <h2 class="panel-heading">"Categories"</h2>
            <div class="panel-list">
                {all_categories.into_iter().map(|cat| {
                    view! {
                        <div class="panel-list-item">
                            <i class="far fa-folder" style="margin-right:6px;font-size:11px;"></i>
                            {cat}
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </section>

        <section class="panel-section">
            <h2 class="panel-heading">"Trending Tags"</h2>
            <div class="tag-cloud">
                {all_tags.into_iter().map(|t| {
                    view! { <span class="tag">{t}</span> }
                }).collect::<Vec<_>>()}
            </div>
        </section>
    }
}

// ============================================
// Scroll to Top
// ============================================

#[component]
fn ScrollTop() -> impl IntoView {
    let (visible, set_visible) = signal(false);

    // Keep interval alive for component lifetime
    let _scroll_interval;
    {
        _scroll_interval = gloo_timers::callback::Interval::new(200, move || {
            if let Some(win) = window() {
                if let Ok(scroll_y) = win.scroll_y() {
                    set_visible.set(scroll_y > 300.0);
                }
            }
        });
    }

    let scroll_to_top = move |_| {
        if let Some(win) = window() {
            let _ = win.scroll_to_with_x_and_y(0.0, 0.0);
        }
    };

    view! {
        <button
            class=move || {
                if visible.get() { "scroll-top visible" } else { "scroll-top" }
            }
            on:click=scroll_to_top
            aria-label="Scroll to top"
        >
            <i class="fas fa-angle-up"></i>
        </button>
    }
}

// ============================================
// Entry Point
// ============================================

#[wasm_bindgen(start)]
pub fn main() {
    leptos::mount::mount_to_body(App);
}

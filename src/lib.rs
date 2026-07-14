use leptos::either::Either;
use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::window;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

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
// Markdown to HTML Renderer
// ============================================

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn render_inline(text: &str) -> String {
    let mut result = text.to_string();

    result = simple_regex_replace(&result, "`([^`]+)`", |caps| {
        format!("<code>{}</code>", &caps[1])
    });

    result = simple_regex_replace(&result, r"!\[([^\]]*)\]\(([^)]+)\)", |caps| {
        format!(
            "<img src=\"{}\" alt=\"{}\" loading=\"lazy\">",
            &caps[2], &caps[1]
        )
    });

    result = simple_regex_replace(&result, r"\[([^\]]+)\]\(([^)]+)\)", |caps| {
        format!(
            "<a href=\"{}\" target=\"_blank\" rel=\"noopener\">{}</a>",
            &caps[2], &caps[1]
        )
    });

    result = simple_regex_replace(&result, r"\*\*([^*]+)\*\*", |caps| {
        format!("<strong>{}</strong>", &caps[1])
    });

    result = simple_regex_replace_italic(&result);

    result
}

fn simple_regex_replace(
    input: &str,
    _pattern: &str,
    f: impl Fn(Vec<&str>) -> String,
) -> String {
    match _pattern {
        "`([^`]+)`" => {
            let mut out = String::new();
            let chars: Vec<char> = input.chars().collect();
            let len = chars.len();
            let mut i = 0;
            while i < len {
                if chars[i] == '`' {
                    if let Some(end) = chars[i + 1..].iter().position(|&c| c == '`') {
                        let content: String = chars[i + 1..i + 1 + end].iter().collect();
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
                if let Some(close_bracket) = s[abs_bang + 2..].find(']') {
                    let alt = &s[abs_bang + 2..abs_bang + 2 + close_bracket];
                    let rest = &s[abs_bang + 2 + close_bracket + 1..];
                    if rest.starts_with('(') {
                        if let Some(close_paren) = rest[1..].find(')') {
                            let url = &rest[1..1 + close_paren];
                            out.push_str(&f(vec![alt, url]));
                            pos = abs_bang + 2 + close_bracket + 1 + 1 + close_paren + 1;
                            continue;
                        }
                    }
                    out.push_str(&s[abs_bang..abs_bang + 2 + close_bracket + 1]);
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
                if abs_bracket > 0 && s.as_bytes()[abs_bracket - 1] == b'!' {
                    out.push_str(&s[pos..abs_bracket + 1]);
                    pos = abs_bracket + 1;
                    continue;
                }
                out.push_str(&s[pos..abs_bracket]);
                if let Some(close_bracket) = s[abs_bracket + 1..].find(']') {
                    let text = &s[abs_bracket + 1..abs_bracket + 1 + close_bracket];
                    let rest = &s[abs_bracket + 1 + close_bracket + 1..];
                    if rest.starts_with('(') {
                        if let Some(close_paren) = rest[1..].find(')') {
                            let url = &rest[1..1 + close_paren];
                            out.push_str(&f(vec![text, url]));
                            pos =
                                abs_bracket + 1 + close_bracket + 1 + 1 + close_paren + 1;
                            continue;
                        }
                    }
                    out.push_str(&s[abs_bracket..abs_bracket + 1 + close_bracket + 1]);
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
                if let Some(close_star) = s[abs_star + 2..].find("**") {
                    let content = &s[abs_star + 2..abs_star + 2 + close_star];
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
        if chars[i] == '*' && (i == 0 || chars[i - 1] != '*') {
            if let Some(star_end) = chars[i + 1..].iter().position(|&c| c == '*') {
                let abs_end = i + 1 + star_end;
                if abs_end + 1 < len && chars[abs_end + 1] == '*' {
                    out.push(chars[i]);
                    i += 1;
                    continue;
                }
                if star_end > 0 && chars[i + star_end] == '*' {
                    out.push(chars[i]);
                    i += 1;
                    continue;
                }
                let content: String = chars[i + 1..abs_end].iter().collect();
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
            html.push_str("<p>&nbsp;</p>\n");
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
            html.push_str(&format!(
                "<blockquote>{}</blockquote>\n",
                render_inline(rest)
            ));
        } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            html.push_str("<ul>\n");
            let first = &trimmed[2..];
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

fn highlight_code(code: &str, lang: &str) -> String {
    let escaped = escape_html(code);
    match lang {
        "rust" | "rs" => highlight_keywords(
            &escaped,
            &[
                "fn", "let", "mut", "pub", "use", "mod", "struct", "enum", "impl",
                "trait", "where", "match", "if", "else", "for", "while", "loop",
                "return", "async", "await", "move", "self", "Self", "super", "crate",
                "const", "static", "type", "ref", "as", "in", "dyn", "unsafe", "extern",
                "true", "false",
            ],
        ),
        "toml" => highlight_keywords(&escaped, &["true", "false"]),
        "bash" | "sh" | "shell" => highlight_keywords(
            &escaped,
            &[
                "echo", "cd", "ls", "mkdir", "cargo", "git", "npm", "curl", "sudo",
                "rustup", "trunk", "cat",
            ],
        ),
        "json" => highlight_keywords(&escaped, &["true", "false", "null"]),
        "yaml" | "yml" => highlight_keywords(&escaped, &["true", "false", "null"]),
        "html" | "css" => highlight_keywords(&escaped, &[]),
        _ => format!("<code>{}</code>", escaped),
    }
}

fn highlight_keywords(code: &str, keywords: &[&str]) -> String {
    let mut result = code.to_string();

    for kw in keywords {
        let word_boundary = format!(" {} ", kw);
        let replacement = format!(" <span class=\"token-keyword\">{}</span> ", kw);
        result = result.replace(&word_boundary, &replacement);
    }

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

    let mut num_result = String::new();
    let chars: Vec<char> = result.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if chars[i].is_ascii_digit()
            && (i == 0 || !chars[i - 1].is_alphanumeric())
        {
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
                    let _ =
                        storage.set_item("theme", if is_dark { "dark" } else { "light" });
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
        if h.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", h)
        }
    };

    let (route_read, route_write) = signal(parse_hash());
    let set_route = route_write;

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

    let toggle_dark = move |is_dark: bool| {
        set_dark_mode.set(is_dark);
        apply_theme(is_dark);
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
                    if sidebar_open.get() { "sidebar-overlay active" } else { "sidebar-overlay" }
                }
                on:click=close_sidebar
            />

            <aside class=move || {
                if sidebar_open.get() { "sidebar open" } else { "sidebar" }
            }>
                <div class="sidebar-inner">
                    <div class="sidebar-header">
                        <div class="avatar">"G"</div>
                        <div class="site-title">"guilt92"</div>
                        <div class="site-subtitle">"Low-level engineering, demystified."</div>
                    </div>

                    <nav class="sidebar-nav">
                        <div class="nav-section-title">"Menu"</div>

                        <NavItem route=route_read href="/".to_string() icon="fa-solid fa-house".to_string() label="Home".to_string() close=close_sidebar />
                        <NavItem route=route_read href="/archives".to_string() icon="fa-solid fa-archive".to_string() label="Archives".to_string() close=close_sidebar />
                        <NavItem route=route_read href="/categories".to_string() icon="fa-solid fa-folder-tree".to_string() label="Categories".to_string() close=close_sidebar />
                        <NavItem route=route_read href="/tags".to_string() icon="fa-solid fa-tags".to_string() label="Tags".to_string() close=close_sidebar />
                        <NavItem route=route_read href="/about".to_string() icon="fa-solid fa-circle-info".to_string() label="About".to_string() close=close_sidebar />
                    </nav>

                    <div class="sidebar-footer">
                        <div class="social-links">
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
                        <div class="theme-toggle-wrapper">
                            <div class="theme-toggle">
                                <button
                                    class=move || {
                                        if dark_mode.get() { "theme-toggle-btn active" } else { "theme-toggle-btn" }
                                    }
                                    on:click=move |_| toggle_dark(true)
                                >
                                    <i class="fas fa-moon"></i>
                                </button>
                                <button
                                    class=move || {
                                        if !dark_mode.get() { "theme-toggle-btn active" } else { "theme-toggle-btn" }
                                    }
                                    on:click=move |_| toggle_dark(false)
                                >
                                    <i class="fas fa-sun"></i>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </aside>

            <div class="main-wrapper">
                <header class="topbar">
                    <button class="sidebar-toggle" on:click=toggle_sidebar aria-label="Toggle sidebar">
                        <i class="fas fa-bars"></i>
                    </button>
                    <div class="topbar-title">"guilt92"</div>
                    <div class="topbar-actions">
                        <button class="sidebar-toggle" on:click=move |_| toggle_dark(!dark_mode.get()) aria-label="Toggle theme">
                            {move || {
                                if dark_mode.get() {
                                    view! { <i class="fas fa-sun"></i> }
                                } else {
                                    view! { <i class="fas fa-moon"></i> }
                                }
                            }}
                        </button>
                    </div>
                </header>

                <div class="content-area">
                    <div class="content-inner">
                        <main class="main-col">
                            {move || {
                                let r = route_read.get();
                                let p = posts.clone();
                                if r == "/about" {
                                    Either::Left(Either::Left(view! { <AboutPage /> }))
                                } else if r == "/archives" {
                                    Either::Left(Either::Right(view! { <ArchivesPage posts=p /> }))
                                } else if r == "/categories" {
                                    Either::Right(Either::Left(view! { <CategoriesPage posts=p /> }))
                                } else if r == "/tags" {
                                    Either::Right(Either::Right(Either::Left(view! { <TagsPage posts=p /> })))
                                } else if r.starts_with("/post/") {
                                    let slug = r.trim_start_matches("/post/").to_string();
                                    Either::Right(Either::Right(Either::Right(Either::Left(view! { <PostPage slug=slug posts=p /> }))))
                                } else {
                                    Either::Right(Either::Right(Either::Right(Either::Right(view! { <HomePage posts=p _search_query=String::new() /> }))))
                                }
                            }}
                        </main>
                    </div>

                    <footer class="site-footer">
                        <p>
                            "© 2026 guilt92 · Built with "
                            <a href="https://leptos.dev" target="_blank" rel="noopener">"Leptos"</a>
                            " + "
                            <a href="https://www.rust-lang.org" target="_blank" rel="noopener">"Rust"</a>
                            " · "
                            <a href="https://github.com/guilt92/guilt92.github.io" target="_blank" rel="noopener">"Source"</a>
                        </p>
                    </footer>
                </div>
            </div>

            <ScrollTop />
        </div>
    }
}

// ============================================
// NavItem
// ============================================

#[component]
fn NavItem<F>(
    route: ReadSignal<String>,
    href: String,
    icon: String,
    label: String,
    close: F,
) -> impl IntoView
where
    F: Fn(web_sys::MouseEvent) + 'static,
{
    let href_clone = href.clone();
    let is_active = move || {
        let r = route.get();
        if href_clone == "/" {
            r == "/" || r.is_empty()
        } else {
            r == href_clone
        }
    };

    view! {
        <div class="nav-item">
            <a
                class=move || {
                    if is_active() { "nav-link active" } else { "nav-link" }
                }
                href=format!("#{}", href)
                on:click=close
            >
                <i class=icon></i>
                <span>{label}</span>
            </a>
        </div>
    }
}

// ============================================
// Home Page
// ============================================

#[component]
fn HomePage(posts: Vec<BlogPost>, _search_query: String) -> impl IntoView {
    let mut sorted = posts.clone();
    sorted.sort_by(|a, b| b.date.cmp(&a.date));

    let (query, set_query) = signal(String::new());

    let filtered = move || {
        let q = query.get().to_lowercase();
        if q.is_empty() {
            sorted.clone()
        } else {
            sorted
                .iter()
                .filter(|p| {
                    p.title.to_lowercase().contains(&q)
                        || p.excerpt.to_lowercase().contains(&q)
                        || p.tags.iter().any(|t| t.to_lowercase().contains(&q))
                        || p.category.to_lowercase().contains(&q)
                })
                .cloned()
                .collect()
        }
    };

    view! {
        <div>
            <h1 class="page-heading">"guilt92"</h1>
            <p class="page-subheading">
                "Deep technical notes on systems programming, distributed systems, performance engineering, and software architecture."
            </p>

            <div class="search-wrapper">
                <i class="fas fa-search search-icon"></i>
                <input
                    class="search-input"
                    type="search"
                    placeholder="Search posts..."
                    aria-label="Search posts"
                    prop:value=move || query.get()
                    on:input=move |ev| {
                        if let Some(target) = ev.target() {
                            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                                set_query.set(input.value());
                            }
                        }
                    }
                />
            </div>

            <section class="post-list">
                {move || {
                    let filtered = filtered();
                    if filtered.is_empty() {
                        Either::Left(view! {
                            <div class="empty-state">
                                <i class="fas fa-search"></i>
                                <p>"No posts found matching your search."</p>
                            </div>
                        })
                    } else {
                        Either::Right(view! {
                            {filtered.into_iter().enumerate().map(|(i, post)| {
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
                                    <a href=format!("#/post/{}", slug) class=class_str>
                                        <div class="post-card-header">
                                            <span class="post-card-category">{category}</span>
                                            <span class="post-card-date"><i class="far fa-calendar"></i> {date}</span>
                                        </div>
                                        <h2 class="post-card-title">{title}</h2>
                                        <p class="post-card-excerpt">{excerpt}</p>
                                        <div class="post-card-tags">
                                            {tags.into_iter().map(|t| {
                                                view! { <span class="post-card-tag">"#" {t}</span> }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    </a>
                                }
                            }).collect::<Vec<_>>()}
                        })
                    }
                }}
            </section>
        </div>
    }
}

// ============================================
// Archives Page
// ============================================

#[component]
fn ArchivesPage(posts: Vec<BlogPost>) -> impl IntoView {
    let mut sorted = posts.clone();
    sorted.sort_by(|a, b| b.date.cmp(&a.date));

    let mut years: Vec<(String, Vec<BlogPost>)> = Vec::new();
    for post in &sorted {
        let year = if post.date.len() >= 4 {
            post.date[..4].to_string()
        } else {
            "Unknown".to_string()
        };
        if let Some(last) = years.last_mut() {
            if last.0 == year {
                last.1.push(post.clone());
                continue;
            }
        }
        years.push((year, vec![post.clone()]));
    }

    view! {
        <div class="archives-page animate-in">
            <h1><i class="fas fa-archive"></i> "Archives"</h1>

            {years.into_iter().map(|(year, year_posts)| {
                view! {
                    <div class="archive-year">
                        <h2 class="archive-year-title">{year}</h2>
                        <div class="archive-list">
                            {year_posts.into_iter().map(|post| {
                                let slug = post.slug.clone();
                                let title = post.title.clone();
                                let date = post.date.clone();
                                let category = post.category.clone();
                                view! {
                                    <a href=format!("#/post/{}", slug) class="archive-item">
                                        <span class="archive-date">{date}</span>
                                        <span class="archive-title">{title}</span>
                                        <span class="archive-category">{category}</span>
                                    </a>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

// ============================================
// Categories Page
// ============================================

#[component]
fn CategoriesPage(posts: Vec<BlogPost>) -> impl IntoView {
    let mut categories: Vec<(String, Vec<BlogPost>)> = Vec::new();
    for post in &posts {
        if let Some(last) = categories.last_mut() {
            if last.0 == post.category {
                last.1.push(post.clone());
                continue;
            }
        }
        categories.push((post.category.clone(), vec![post.clone()]));
    }
    categories.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    view! {
        <div class="categories-page animate-in">
            <h1><i class="fas fa-folder-tree"></i> "Categories"</h1>

            {categories.into_iter().map(|(cat, cat_posts)| {
                view! {
                    <div class="category-group">
                        <h2 class="category-title">
                            <i class="fas fa-folder"></i>
                            {cat.clone()}
                            <span style="font-size:13px;color:var(--text-muted);font-weight:400;">
                                ({cat_posts.len()})
                            </span>
                        </h2>
                        <div class="archive-list">
                            {cat_posts.into_iter().map(|post| {
                                let slug = post.slug.clone();
                                let title = post.title.clone();
                                let date = post.date.clone();
                                view! {
                                    <a href=format!("#/post/{}", slug) class="archive-item">
                                        <span class="archive-date">{date}</span>
                                        <span class="archive-title">{title}</span>
                                    </a>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

// ============================================
// Tags Page
// ============================================

#[component]
fn TagsPage(posts: Vec<BlogPost>) -> impl IntoView {
    let mut tag_counts: Vec<(String, usize)> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for post in &posts {
        for tag in &post.tags {
            if seen.insert(tag.clone()) {
                let count = posts
                    .iter()
                    .filter(|p| p.tags.contains(tag))
                    .count();
                tag_counts.push((tag.clone(), count));
            }
        }
    }
    tag_counts.sort_by(|a, b| b.1.cmp(&a.1));

    view! {
        <div class="tags-page animate-in">
            <h1><i class="fas fa-tags"></i> "Tags"</h1>

            <div class="tag-cloud">
                {tag_counts.into_iter().map(|(tag, count)| {
                    let size = if count >= 4 { "1.1em" } else if count >= 3 { "1em" } else if count >= 2 { "0.95em" } else { "0.85em" };
                    view! {
                        <span class="tag-item" style=format!("font-size:{}", size)>
                            "#" {tag}
                            <span class="tag-count">({count})</span>
                        </span>
                    }
                }).collect::<Vec<_>>()}
            </div>
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
        None => Either::Left(view! {
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
                            <div class="post-card-header" style="margin-bottom:12px;">
                                <span class="post-card-category">{p.category.clone()}</span>
                            </div>
                            <h1 class="post-title">{p.title.clone()}</h1>
                            <div class="post-meta">
                                <span><i class="far fa-calendar"></i>{p.date.clone()}</span>
                                <span><i class="far fa-folder"></i>{p.category.clone()}</span>
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
            <h1>"About"</h1>
            <div class="about-card">
                <p>
                    "A technical space focused on deep-dive analysis of systems programming, performance engineering, "
                    "and software architecture. Built entirely in Rust and compiled to WebAssembly."
                </p>
            </div>

            <h2>"Focus Areas"</h2>
            <ul style="padding-left:20px;list-style:disc;">
                <li style="margin-bottom:8px;color:var(--text-secondary);">"Rust language internals and advanced patterns"</li>
                <li style="margin-bottom:8px;color:var(--text-secondary);">"WebAssembly and browser-level systems programming"</li>
                <li style="margin-bottom:8px;color:var(--text-secondary);">"Concurrent and async programming architectures"</li>
                <li style="margin-bottom:8px;color:var(--text-secondary);">"Performance engineering and optimization"</li>
                <li style="margin-bottom:8px;color:var(--text-secondary);">"Low-level systems and OS internals"</li>
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
// Scroll to Top
// ============================================

#[component]
fn ScrollTop() -> impl IntoView {
    let (visible, set_visible) = signal(false);

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

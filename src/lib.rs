use leptos::either::Either;
use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::{window, HashChangeEvent};

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    // Remove fallback loading splash before mounting the app
    if let Some(doc) = window().and_then(|w| w.document()) {
        if let Some(splash) = doc.get_element_by_id("loading-splash") {
            splash.remove();
        }
    }
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

#[derive(Clone, Debug, PartialEq)]
struct GitHubProject {
    name: String,
    description: String,
    html_url: String,
    language: Option<String>,
    topics: Vec<String>,
    homepage: Option<String>,
    updated_at: String,
}

fn format_date(date: &str) -> String {
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() < 3 { return date.to_string(); }
    let months = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];
    let month_idx: usize = parts[1].parse::<usize>().unwrap_or(1).saturating_sub(1);
    let day = &parts[2];
    let day_clean = if day.len() >= 2 { &day[..2] } else { day };
    format!("{} {}, {}", months.get(month_idx).unwrap_or(&"???"), day_clean, parts[0])
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
        // BlogPost {
        //     title: "Deploying Rust WASM to GitHub Pages".into(),
        //     slug: "deploying-rust-wasm-github-pages".into(),
        //     date: "2025-09-18".into(),
        //     excerpt: "Complete CI/CD pipeline for building, optimizing, and deploying a Leptos WASM application to GitHub Pages with Trunk.".into(),
        //     category: "DevOps".into(),
        //     tags: vec!["rust".into(), "wasm".into(), "ci-cd".into(), "github-actions".into()],
        //     content: include_str!("../content/posts/deploying-rust-wasm-github-pages.md").into(),
        // },
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
// GitHub Projects Data
// ============================================

fn all_projects() -> Vec<GitHubProject> {
    vec![
        GitHubProject {
            name: "guilt92.github.io".into(),
            description: "Personal blog built with Rust, Leptos, and WebAssembly — deploying engineering field notes on GitHub Pages.".into(),
            html_url: "https://github.com/guilt92/guilt92.github.io".into(),
            language: Some("Rust".into()),
            topics: vec!["rust".into(), "leptos".into(), "wasm".into(), "blog".into()],
            homepage: Some("https://guilt92.github.io".into()),
            updated_at: "2025-12-15".into(),
        },
        GitHubProject {
            name: "shortlink".into(),
            description: "A Rust-based URL shortener with async HTTP handling using Tokio.".into(),
            html_url: "https://github.com/Guilt92/shortLink".into(),
            language: Some("Rust".into()),
            topics: vec!["rust".into(), "tokio".into(), "async".into(), "web".into()],
            homepage: None,
            updated_at: "2025-08-20".into(),
        },
        GitHubProject {
            name: "OutisCloud-hickory-dns".into(),
            description: "A Rust DNS client and server implementation using the Hickory DNS library.".into(),
            html_url: "https://github.com/Guilt92/OutisCloud-hickory-dns".into(),
            language: Some("Rust".into()),
            topics: vec!["rust".into(), "dns".into(), "networking".into(), "async".into()],
            homepage: None,
            updated_at: "2025-06-10".into(),
        },
        GitHubProject {
            name: "NameShift".into(),
            description: "DNS server management automation script for declarative zone configuration.".into(),
            html_url: "https://github.com/Guilt92/NameShift".into(),
            language: Some("Shell".into()),
            topics: vec!["shell".into(), "dns".into(), "automation".into(), "linux".into()],
            homepage: None,
            updated_at: "2025-04-18".into(),
        },
        GitHubProject {
            name: "FireRust".into(),
            description: "Rust firewall rules manager with NFTables backend for Linux servers.".into(),
            html_url: "https://github.com/Guilt92/FireRust".into(),
            language: Some("Rust".into()),
            topics: vec!["rust".into(), "firewall".into(), "networking".into(), "linux".into()],
            homepage: None,
            updated_at: "2025-03-05".into(),
        },
        GitHubProject {
            name: "NetMon".into(),
            description: "Lightweight network monitoring tool tracking bandwidth, latency, and service uptime.".into(),
            html_url: "https://github.com/Guilt92/NetMon".into(),
            language: Some("Shell".into()),
            topics: vec!["shell".into(), "monitoring".into(), "network".into(), "linux".into()],
            homepage: None,
            updated_at: "2025-01-22".into(),
        },
        GitHubProject {
            name: "OutisCloud-Core-CDN".into(),
            description: "Core CDN infrastructure manager for distributing content across edge nodes.".into(),
            html_url: "https://github.com/Guilt92/OutisCloud-Core-CDN".into(),
            language: Some("Rust".into()),
            topics: vec!["rust".into(), "cdn".into(), "networking".into(), "infrastructure".into()],
            homepage: None,
            updated_at: "2024-11-30".into(),
        },
        GitHubProject {
            name: "Ytiruces".into(),
            description: "NFTables rule management script for declarative firewall policy definition.".into(),
            html_url: "https://github.com/Guilt92/Ytiruces".into(),
            language: Some("Shell".into()),
            topics: vec!["shell".into(), "nftables".into(), "firewall".into(), "linux".into()],
            homepage: None,
            updated_at: "2024-09-14".into(),
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
        format!("<code>{}</code>", &caps[0])
    });

    result = simple_regex_replace(&result, r"!\[([^\]]*)\]\(([^)]+)\)", |caps| {
        format!(
            "<img src=\"{}\" alt=\"{}\" loading=\"lazy\">",
            &caps[1], &caps[0]
        )
    });

    result = simple_regex_replace(&result, r"\[([^\]]+)\]\(([^)]+)\)", |caps| {
        format!(
            "<a href=\"{}\" target=\"_blank\" rel=\"noopener\">{}</a>",
            &caps[1], &caps[0]
        )
    });

    result = simple_regex_replace(&result, r"\*\*([^*]+)\*\*", |caps| {
        format!("<strong>{}</strong>", &caps[0])
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

fn estimate_reading_time(content: &str) -> String {
    let words: Vec<&str> = content.split_whitespace().collect();
    let minutes = (words.len() as f64 / 200.0).ceil() as u32;
    if minutes <= 1 { "1 min read".to_string() } else { format!("{} min read", minutes) }
}

fn extract_toc_items(md: &str) -> Vec<(String, String)> {
    let mut items = Vec::new();
    for line in md.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("## ") {
            let id = rest.to_lowercase().replace(' ', "-").replace(|c: char| !c.is_alphanumeric() && c != '-', "");
            items.push((rest.to_string(), id));
        } else if let Some(rest) = trimmed.strip_prefix("### ") {
            let id = rest.to_lowercase().replace(' ', "-").replace(|c: char| !c.is_alphanumeric() && c != '-', "");
            items.push((rest.to_string(), id));
        }
    }
    items
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
            let id = rest.to_lowercase().replace(' ', "-").replace(|c: char| !c.is_alphanumeric() && c != '-', "");
            html.push_str(&format!("<h4 id=\"{}\">{}</h4>\n", id, render_inline(rest)));
        } else if let Some(rest) = trimmed.strip_prefix("### ") {
            let id = rest.to_lowercase().replace(' ', "-").replace(|c: char| !c.is_alphanumeric() && c != '-', "");
            html.push_str(&format!("<h3 id=\"{}\">{}</h3>\n", id, render_inline(rest)));
        } else if let Some(rest) = trimmed.strip_prefix("## ") {
            let id = rest.to_lowercase().replace(' ', "-").replace(|c: char| !c.is_alphanumeric() && c != '-', "");
            html.push_str(&format!("<h2 id=\"{}\">{}</h2>\n", id, render_inline(rest)));
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

    // Use hashchange event for immediate navigation updates
    let _hashchange_listener = {
        let set_route = set_route;
        let parse_hash = parse_hash;
        let window = window().expect("no window");
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: HashChangeEvent| {
            set_route.set(parse_hash());
        }) as Box<dyn FnMut(_)>);
        window
            .add_event_listener_with_callback("hashchange", closure.as_ref().unchecked_ref())
            .expect("failed to add hashchange listener");
        closure.forget();
    };

    // Also keep polling as fallback
    let _route_interval;
    {
        let set_route = set_route;
        let route_read = route_read;
        _route_interval = gloo_timers::callback::Interval::new(500, move || {
            let new_route = parse_hash();
            let current = route_read.get_untracked();
            if new_route != current {
                set_route.set(new_route);
            }
        });
    }

    let _keydown = {
        let set_sidebar_open = set_sidebar_open;
        if let Some(win) = window() {
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |ev: web_sys::KeyboardEvent| {
                if ev.key() == "Escape" {
                    set_sidebar_open.set(false);
                }
            }) as Box<dyn FnMut(_)>);
            let _ = win.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    };

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
        <a href="#main-content" class="skip-link">Skip to content</a>

        <div class="app-layout">
            <div
                class=move || {
                    if sidebar_open.get() { "sidebar-overlay active" } else { "sidebar-overlay" }
                }
                on:click=close_sidebar
                aria-hidden="true"
            />

            <aside
                class=move || {
                    if sidebar_open.get() { "sidebar open" } else { "sidebar" }
                }
                aria-label="Site navigation"
            >
                <div class="sidebar-inner">
                    <div class="sidebar-header">
                        <div class="brand-mark">
                            <div class="brand-icon">g92</div>
                            <div>
                                <div class="site-title">Amirhosein Allahdadi</div>
                                <div class="site-subtitle">DevOps Engineer</div>
                            </div>
                        </div>
                    </div>

                    <nav class="sidebar-nav">
                        <div class="nav-section-title">"Menu"</div>

                        <NavItem route=route_read href="/".to_string() icon="fa-solid fa-house".to_string() label="Home".to_string() close=close_sidebar />
                        <NavItem route=route_read href="/projects".to_string() icon="fa-solid fa-code".to_string() label="Projects".to_string() close=close_sidebar />
                        <NavItem route=route_read href="/categories".to_string() icon="fa-solid fa-folder-tree".to_string() label="Categories".to_string() close=close_sidebar />
                        <NavItem route=route_read href="/about".to_string() icon="fa-solid fa-circle-info".to_string() label="About".to_string() close=close_sidebar />
                    </nav>

                    <div class="sidebar-footer">
                        <div class="social-links">
                            <a href="https://github.com/guilt92" target="_blank" rel="noopener" class="social-link" aria-label="GitHub">
                                <i class="fab fa-github"></i>
                            </a>
                            <a href="https://twitter.com/Aallahdadi92" target="_blank" rel="noopener" class="social-link" aria-label="Twitter">
                                <i class="fab fa-x-twitter"></i>
                            </a>
                            <a href="mailto:amirhoseinalahdadi76@gmail.com" class="social-link" aria-label="Email">
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
                <header class="topbar" aria-label="Mobile header">
                    <button class="sidebar-toggle" on:click=toggle_sidebar aria-label="Toggle sidebar" aria-expanded=sidebar_open>
                        <i class="fas fa-bars"></i>
                    </button>
                    <div class="topbar-brand">guilt92</div>
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
                                match r.as_str() {
                                    "/about" => view! { <AboutPage /> }.into_any(),
                                    "/projects" => view! { <ProjectsPage /> }.into_any(),
                                    "/categories" => view! { <CategoriesPage posts=p.clone() /> }.into_any(),
                                    _ if r.starts_with("/post/") => {
                                        let slug = r.trim_start_matches("/post/").to_string();
                                        view! { <PostPage slug=slug posts=p /> }.into_any()
                                    }
                                    _ => view! { <HomePage posts=p _search_query=String::new() /> }.into_any(),
                                }
                            }}
                        </main>
                    </div>

                    <footer class="site-footer">
                        <p>guilt92 &mdash; Engineering Field Notes</p>
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
            r == href_clone || r.starts_with(&format!("{}/", href_clone))
        }
    };

    view! {
        <div class="nav-item">
            <a
                class={
                    let value = is_active.clone();
                    move || {
                        let ia = value.clone();
                        if ia() { "nav-link active" } else { "nav-link" }
                    }
                }
                href=format!("#{}", href)
                on:click=close
                aria-current=move || {
                    let ia = is_active.clone();
                    if ia() { Some("page") } else { None }
                }
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
    let post_count = sorted.len();
    let featured_post = sorted.first();
    let recent_posts: Vec<_> = sorted.into_iter().take(7).collect();

    view! {
        <div class="home-container">
            <section class="home-hero">
                <div class="home-hero-content">
                    <div class="home-hero-main">
                        <span class="home-eyebrow">DevOps Engineer</span>
                        <h1 class="home-title">Amirhosein Allahdadi</h1>
                        <p class="home-role">Building resilient infrastructure and writing about systems, Rust, and Linux.</p>
                        <p class="home-desc">
                            Engineering field notes on distributed systems, Rust, Kubernetes, networking, and the craft of building reliable software.
                        </p>
                        <div class="home-links">
                            <a href="https://github.com/guilt92" target="_blank" rel="noopener" class="home-link">
                                <i class="fab fa-github"></i>
                                GitHub
                            </a>
                            <a href="#/about" class="home-link">
                                <i class="fas fa-user"></i>
                                About
                            </a>
                            <a href="#/archives" class="home-link">
                                <i class="fas fa-archive"></i>
                                Archives
                            </a>
                        </div>
                    </div>
                    <aside class="home-hero-aside">
                        <div class="home-stats">
                            <div class="home-stat">
                                <span class="home-stat-value">{post_count}</span>
                                <span class="home-stat-label">Posts</span>
                            </div>
                            <div class="home-stat">
                                <span class="home-stat-value">4</span>
                                <span class="home-stat-label">Categories</span>
                            </div>
                        </div>
                    </aside>
                </div>
            </section>

            <section class="home-posts">
                <div class="posts-section-header">
                    <h2 class="posts-section-title">Recent Posts</h2>
                    <a href="#/archives" class="posts-section-link">View all <i class="fas fa-arrow-right"></i></a>
                </div>
                {if recent_posts.is_empty() {
                    Either::Left(view! {
                        <div class="empty-state">
                            <i class="fas fa-inbox"></i>
                            <p>"No posts yet."</p>
                        </div>
                    })
                } else {
                    Either::Right(view! {
                        <div class="posts-grid">
                            {recent_posts.iter().enumerate().map(|(i, post)| {
                                let slug = post.slug.clone();
                                let title = post.title.clone();
                                let excerpt = post.excerpt.clone();
                                let date = post.date.clone();
                                let formatted_date = format_date(&date);
                                let category = post.category.clone();
                                let is_featured = i == 0;
                                let tags = post.tags.clone();
                                view! {
                                <article class="post-card" class:featured=is_featured>
                                    <span class="post-featured-badge">Featured</span>
                                    <div class="post-card-header">
                                            <div class="post-card-meta">
                                                <span class="post-card-category">{category}</span>
                                                <span class="post-card-dot"></span>
                                                <span class="post-card-date">{formatted_date}</span>
                                            </div>
                                            <h3 class="post-card-title">
                                                <a href=format!("#/post/{}", slug) >{title}</a>
                                            </h3>
                                        </div>
                                        <p class="post-card-excerpt">{excerpt}</p>
                                        <div class="post-card-footer">
                                            <div class="post-card-tags">
                                                {tags.iter().take(3).map(|t| view! {
                                                    <span class="post-card-tag">{format!("#{}", t)}</span>
                                                }).collect::<Vec<_>>()}
                                            </div>
                                            <span class="post-card-read">Read <i class="fas fa-arrow-right"></i></span>
                                        </div>
                                    </article>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    })
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
        <div class="archive-page animate-in">
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

    let category_icons: std::collections::HashMap<&str, &'static str> = [
        ("Rust", "fa-brands fa-rust"),
        ("Systems", "fa-solid fa-gears"),
        ("Fundamentals", "fa-solid fa-book"),
        ("DevOps", "fa-solid fa-cloud"),
    ].into_iter().collect();

    let category_descs: std::collections::HashMap<&str, &'static str> = [
        ("Rust", "Memory safety, concurrency, and zero-cost abstractions"),
        ("Systems", "Operating systems, networking, and distributed systems"),
        ("Fundamentals", "Core programming concepts and patterns"),
        ("DevOps", "Infrastructure, deployment, and automation"),
    ].into_iter().collect();

    view! {
        <div class="categories-page animate-in">
            <header class="page-header">
                <h1><i class="fas fa-folder-tree"></i> Categories</h1>
                <p class="page-desc">Browse posts by topic area</p>
            </header>

            <div class="cat-grid">
                {categories.into_iter().map(|(cat, cat_posts)| {
                    let icon = category_icons.get(cat.as_str()).unwrap_or(&"fa-solid fa-folder").to_string();
                    let desc = category_descs.get(cat.as_str()).unwrap_or(&"Articles and tutorials").to_string();
                    let post_count = cat_posts.len();
                    let cat_slug = cat.to_lowercase().replace(' ', "-");
                    view! {
                        <div class="cat-card" id=format!("cat-{}", cat_slug)>
                            <div class="cat-card-head">
                                <div class="cat-icon-wrap">
                                    <i class={icon}></i>
                                </div>
                                <div class="cat-card-info">
                                    <h2 class="cat-card-name">{cat}</h2>
                                    <p class="cat-card-desc">{desc}</p>
                                </div>
                                <div class="cat-card-count">{post_count}</div>
                            </div>
                            <div class="cat-card-posts">
                                {cat_posts.into_iter().map(|post| {
                                    let slug = post.slug.clone();
                                    let title = post.title.clone();
                                    let date = post.date.clone();
                                    view! {
                                        <a href=format!("#/post/{}", slug) class="cat-card-item">
                                            <span class="cat-card-item-title">{title}</span>
                                            <span class="cat-card-item-date">{date}</span>
                                        </a>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
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
                    let _size = if count >= 4 { "1.1em" } else if count >= 3 { "1em" } else if count >= 2 { "0.95em" } else { "0.85em" };
                    view! {
                        <a href=format!("#/tag/{}", tag.clone()) class="tag-item">
                            <span class="tag-hash">#</span>{tag.clone()}
                            <span class="tag-count">({count})</span>
                        </a>
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
            let toc_items = extract_toc_items(&p.content);
            let reading_time = estimate_reading_time(&p.content);

            let mut sorted = posts.clone();
            sorted.sort_by(|a, b| b.date.cmp(&a.date));
            let current_idx = sorted.iter().position(|x| x.slug == p.slug);
            let prev_post = current_idx.and_then(|i| if i + 1 < sorted.len() { Some((sorted[i + 1].slug.clone(), sorted[i + 1].title.clone())) } else { None });
            let next_post = current_idx.and_then(|i| if i > 0 { Some((sorted[i - 1].slug.clone(), sorted[i - 1].title.clone())) } else { None });

            let has_toc = toc_items.len() > 2;

            Either::Right(view! {
                <div class="article-page animate-in">
                    <div class="reading-progress" id="reading-progress"></div>
                    <header class="article-header">
                        <div class="article-eyebrow">
                            <span class="article-category">{p.category.clone()}</span>
                            <span class="article-sep">-</span>
                            <span class="article-reading-time">{reading_time}</span>
                        </div>
                        <h1 class="article-title">{p.title.clone()}</h1>
                        <div class="article-meta">
                            <span class="article-meta-item"><i class="far fa-calendar"></i> {p.date.clone()}</span>
                        </div>
                        <div class="article-tags">
                            {p.tags.clone().into_iter().map(|t| { let t2 = t.clone(); view! { <a href=format!("#/tag/{}", t) class="tag-chip">{t2}</a> } }).collect::<Vec<_>>()}
                        </div>
                    </header>
                    <div class="article-body" class:has-toc=has_toc>
                        <article class="article-content">
                            <a href="#/" class="back-link">
                                <i class="fas fa-arrow-left"></i>
                                "Back to all posts"
                            </a>
                            <div inner_html=html_content />
                            <nav class="article-nav">
                                {prev_post.map(|(slug, title)| view! {
                                    <a href=format!("#/post/{}", slug) class="article-nav-item article-nav-prev">
                                        <span class="article-nav-label"><i class="fas fa-arrow-left"></i> Previous</span>
                                        <span class="article-nav-title">{title}</span>
                                    </a>
                                })}
                                {next_post.map(|(slug, title)| view! {
                                    <a href=format!("#/post/{}", slug) class="article-nav-item article-nav-next">
                                        <span class="article-nav-label">Next <i class="fas fa-arrow-right"></i></span>
                                        <span class="article-nav-title">{title}</span>
                                    </a>
                                })}
                            </nav>
                        </article>
                        <aside class="article-toc">
                            <div class="toc-card">
                                <div class="toc-title">Table of Contents</div>
                                <nav class="toc-nav">
                                    {toc_items.iter().map(|(level, text)| {
                                        let id = text.to_lowercase().replace(' ', "-");
                                        view! {
                                            <a href=format!("#{}", id) class="toc-item">{text.clone()}</a>
                                        }
                                    }).collect::<Vec<_>>()}
                                </nav>
                            </div>
                        </aside>
                    </div>
                </div>
            })
        }
    }
}

// ============================================
// Projects Page
// ============================================

#[component]
fn ProjectsPage() -> impl IntoView {
    let projects = all_projects();
    
    view! {
        <div class="projects-page">
            <header class="projects-header">
                <h1><i class="fas fa-folder-open"></i> Projects</h1>
                <p>Open-source work across infrastructure, networking, and systems programming.</p>
            </header>

            <div class="project-list">
                {projects.into_iter().map(|project| {
                    let lang_color = get_language_color(&project.language.clone().unwrap_or_default());
                    let name = project.name.clone();
                    let url = project.html_url.clone();
                    let desc = project.description.clone();
                    let lang = project.language.clone().unwrap_or_else(|| "--".to_string());
                    let updated = project.updated_at.clone();
                    let topics = project.topics.clone();
                    let homepage = project.homepage.clone();

                    view! {
                        <div class="project-row">
                            <div class="project-info">
                                <div class="project-row-head">
                                    <a href=url.clone() target="_blank" rel="noopener" class="project-name">{name}</a>
                                </div>
                                <p class="project-desc">{desc}</p>
                                <div class="project-meta">
                                    <span class="project-lang" style=format!("--lang-color: {};", lang_color)>
                                        <span class="project-lang-dot"></span>
                                        {lang}
                                    </span>
                                    <span class="project-updated">{updated}</span>
                                    <div class="project-topics">
                                        {topics.into_iter().map(|t| {
                                            view! { <a href=format!("#/tag/{}", t.clone()) class="project-topic">#{t.clone()}</a> }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            </div>
                            <div class="project-links">
                                <a href=url.clone() target="_blank" rel="noopener" class="project-btn project-btn-primary">
                                    <i class="fab fa-github"></i>
                                    Source
                                </a>
                                {homepage.map(|h| {
                                    view! {
                                        <a href=h target="_blank" rel="noopener" class="project-btn">
                                            <i class="fas fa-external-link-alt"></i>
                                            Live
                                        </a>
                                    }
                                })}
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

fn get_language_color(lang: &str) -> &'static str {
    match lang.to_lowercase().as_str() {
        "rust" => "#dea584",
        "javascript" | "typescript" => "#f1e05a",
        "python" => "#3572a5",
        "go" => "#00add8",
        "c++" | "cpp" | "c" => "#f34b7d",
        "java" => "#b07219",
        "html" | "css" => "#e34c26",
        "shell" | "bash" => "#89e051",
        "toml" => "#9c4221",
        "yaml" | "yml" => "#cb171e",
        "json" => "#292929",
        "markdown" | "md" => "#083fa1",
        "dockerfile" | "docker" => "#384d54",
        _ => "#8b949e",
    }
}

// ============================================
// About Page
// ============================================

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <div class="about-page animate-in">
            <header class="page-header">
                <h1><i class="fas fa-user"></i> About</h1>
            </header>

            <div class="about-content">
                <section class="about-main">
                    <p class="about-lead">
                        DevOps engineer with a focus on Rust, Linux, systems programming, and infrastructure.
                        I build software that explores how systems work internally, and learn by creating real projects.
                    </p>

                    <p>
                        This website is where I document what I am learning, share personal projects, and write about topics
                        I find interesting - from Rust and Linux to networking, distributed systems, and software architecture.
                    </p>

                    <p>
                        I use this site to keep track of what I learn, document projects, and share practical knowledge.
                        Everything here reflects my learning journey and the projects I am actively working on.
                    </p>
                </section>

                <section class="about-sidebar">
                    <div class="about-topics-card">
                        <h2 class="about-section-title">Topics</h2>
                        <div class="about-topics-list">
                            <div class="about-topic-row">
                                <i class="fa-brands fa-rust"></i>
                                <span>Rust and systems programming</span>
                            </div>
                            <div class="about-topic-row">
                                <i class="fa-solid fa-server"></i>
                                <span>Linux and OS internals</span>
                            </div>
                            <div class="about-topic-row">
                                <i class="fa-solid fa-network-wired"></i>
                                <span>Networking, DNS, and infrastructure</span>
                            </div>
                            <div class="about-topic-row">
                                <i class="fa-solid fa-rocket"></i>
                                <span>Performance and architecture</span>
                            </div>
                            <div class="about-topic-row">
                                <i class="fa-solid fa-code"></i>
                                <span>Personal projects and experiments</span>
                            </div>
                        </div>
                    </div>

                    <div class="about-links-card">
                        <h2 class="about-section-title">Connect</h2>
                        <div class="about-links">
                            <a href="https://github.com/guilt92" target="_blank" rel="noopener" class="about-link">
                                <i class="fab fa-github"></i>
                                "GitHub"
                            </a>
                            <a href="https://x.com/Aallahdadi92" target="_blank" rel="noopener" class="about-link">
                                <i class="fab fa-x-twitter"></i>
                                "Twitter / X"
                            </a>
                            <a href="mailto:amirhoseinalahdadi76@gmail.com" class="about-link">
                                <i class="fas fa-envelope"></i>
                                "Email"
                            </a>
                        </div>
                    </div>
                </section>
            </div>
        </div>
    }
}


// ============================================
// Tag Filter Page
// ============================================

#[component]
fn TagPage(tag: String, posts: Vec<BlogPost>) -> impl IntoView {
    let filtered: Vec<BlogPost> = posts
        .into_iter()
        .filter(|p| p.tags.iter().any(|t| t == &tag))
        .collect();

    let formatted_tag = format!("#{}", tag);

    view! {
        <div class="tag-posts-page">
            <div class="tag-posts-header">
                <a href="#/tags" class="back-link">
                    <i class="fas fa-arrow-left"></i>
                    All tags
                </a>
                <h2>{formatted_tag}</h2>
            </div>

            {if filtered.is_empty() {
                Either::Left(view! {
                    <div class="empty-state">
                        <i class="fas fa-tag"></i>
                        <p>"No posts with this tag yet."</p>
                    </div>
                })
            } else {
                Either::Right(view! {
                    <div class="archive-list">
                        {filtered.into_iter().map(|post| {
                            let slug = post.slug.clone();
                            let title = post.title.clone();
                            let category = post.category.clone();
                            view! {
                                <a href=format!("#/post/{}", slug) class="archive-item">
                                    <span class="archive-title">{title}</span>
                                    <span class="archive-category">{category}</span>
                                </a>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                })
            }}
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

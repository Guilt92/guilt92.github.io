use axum::response::Html;
use axum::{routing::*, Router};
use chrono::Utc;
use maud::Markup;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::debug!("listen on {}", addr);
    let tcp = TcpListener::bind(&addr).await.unwrap();

    axum::serve(tcp, app).await.unwrap();
}
async fn root() -> Html<String> {
    outer_template(maud::html! {
        h3 class="text-4xl mb-4 font-bold animate-pulse" { "OuTiS92" }

    })
}

fn outer_template(body: Markup) -> Html<String> {
    Html(
        maud::html! {
            script src="https://cdn.tailwindcss.com/3.4.16" {}
            body class="flex flex-col items-center justify-center h-screen bg-black text-white" {
                (body)
            }
        }
        .into_string(),
    )
}

fn now_template(title: &str) -> Html<String> {
    let now = Utc::now();

    outer_template(maud::html! {
        h1 class="text-6xl" { (title) }
        p class="text-4xl" { (now) }

        a class="text-blue-400 pt-16 text-xl" href="/" { "Go back home" }
    })
}

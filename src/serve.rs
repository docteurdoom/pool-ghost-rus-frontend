use axum::response::{Html, Json};
use tower_http::services::ServeDir;

use std::net::SocketAddr;

use axum::{routing::get, Router};
use serde_json::Value;
use crate::parse;
use crate::pages;

const HTTP_PORT: u16 = 8080;

fn make_router() -> Router {
    return Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(home))
        .route("/en", get(en))
        .route("/api", get(api));
}

pub async fn http() {
    info!("Serving HTTP ...");
    let addr = SocketAddr::from(([0, 0, 0, 0], HTTP_PORT));
    let app = make_router();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> Html<String> {
    use dioxus::prelude::*;

    debug!("Serving / to anon ...");
    let mut app = VirtualDom::new(pages::home);
	let _ = app.rebuild();
    Html(dioxus_ssr::render(&app))
}

async fn en() -> Html<String> {
    use dioxus::prelude::*;

    debug!("Serving /en to anon ...");
    let mut app = VirtualDom::new(pages::en);
	let _ = app.rebuild();
    Html(dioxus_ssr::render(&app))
}

async fn api() -> Json<Value> {
    use parse::raw_data;
    debug!("Serving /api to anon ...");
    match raw_data() {
        Ok(data) => {
            Json(data)
        }
        Err(e) => {
            error!("Can't read JSON file: {}", e);
            Json(serde_json::from_str("API Error. Please contact server admin.").unwrap())
        }
    }
}

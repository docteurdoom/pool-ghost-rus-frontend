use axum::response::{Html, Json};
use tower_http::services::ServeDir;

use axum_server::tls_rustls::RustlsConfig;
use std::{net::SocketAddr, path::PathBuf};

use axum::{routing::get, Router};
use serde_json::Value;
use crate::parse;

const HTTP_PORT: u16 = 80;
const HTTPS_PORT: u16 = 443;
const CERT_PUB: &str = "./certs/fullchain.pem";
const CERT_PRIV: &str = "./certs/privkey.pem";

fn make_router() -> Router {
    return Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(home))
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

pub async fn https() {
    info!("Serving HTTPS ...");
	let config = RustlsConfig::from_pem_file(
        PathBuf::from(CERT_PUB),
        PathBuf::from(CERT_PRIV),
    ).await.unwrap();
    let app = make_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], HTTPS_PORT));
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> Html<String> {
    use crate::pages;
    use dioxus::prelude::*;

    debug!("Serving / to anon ...");
    let mut app = VirtualDom::new(pages::home);
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

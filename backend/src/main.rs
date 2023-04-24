use std::path::PathBuf;

use axum::{routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum(
    // Name your static assets folder by passing `folder = <name>` to `StaticFolder`
    // If you don't pass a name, it will default to `static`.
    #[shuttle_static_folder::StaticFolder(folder = "public")] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/hello", get(hello_world))
        .nest_service("/", ServeDir::new(static_folder).not_found_service(ServeFile::new("index.html")));

    Ok(router.into())
}

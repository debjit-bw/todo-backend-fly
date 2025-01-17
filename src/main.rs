mod todo_canister;
use std::net::SocketAddr;
use todo_canister::{get_todos, initialize, toggle_todo_by_id};
// use anyhow::Result;
use tower_http::cors::{CorsLayer, Any};

use axum::{Router, routing::get, extract::Path};
use tokio::net::TcpListener;

use serde_json::json;
use axum::Json;

async fn hello_world() -> &'static str {
    "Hello world!"
}

#[tokio::main]
async fn main() {
    // Initialize the agent and principal
    initialize().expect("Failed to initialize agent");

    // Create a CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any) // Adjust this as needed for your use case
        .allow_methods(Any)
        .allow_headers(Any);

    // let todos = get_todos().await;
    // for todo in &todos {
    //     println!("{:?}", todo);
    // }

    // let todos = json!(todos.clone());

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/todos", get(|| async {
            Json(json!(get_todos().await))
        }))
        .route("/toggle/:id", get(|Path(id): Path<u64>| async move {
            Json(json!(toggle_todo_by_id(id).await))
        }))
        // .route("/add", get(|| async { Json(add_todo().await) }))
        .layer(cors);
    let addr = SocketAddr::from(([0,0,0,0], 8080));
    let tcp = TcpListener::bind(&addr).await.unwrap();

    println!("Listening on {}", addr);
    axum::serve(tcp, router).await.unwrap();

    // Ok(())
}

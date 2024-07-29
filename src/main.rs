mod todo_canister;
use std::net::SocketAddr;
use todo_canister::{add_todo, get_todos, initialize};
// use anyhow::Result;

use axum::{Router, routing::get};
use tokio::net::TcpListener;

use serde_json::{json, Value};
use axum::Json;

async fn hello_world() -> &'static str {
    "Hello world!"
}

#[tokio::main]
async fn main() {
    // Initialize the agent and principal
    initialize().expect("Failed to initialize agent");

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
        // .route("/add", get(|| async { Json(add_todo().await) }))
        ;
    let addr = SocketAddr::from(([0,0,0,0], 8080));
    let tcp = TcpListener::bind(&addr).await.unwrap();

    println!("Listening on {}", addr);
    axum::serve(tcp, router).await.unwrap();

    // Ok(())
}

use std::sync::LazyLock;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use routes::{create_task, delete_task, get_all_tasks, get_task, update_task};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tokio::net::TcpListener;

mod error;
mod routes;

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Database setup
    DB.connect::<Ws>("localhost:8000").await?;

    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    DB.use_ns("test").use_db("test").await?;

    // Router setup
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let app = Router::new()
        .route("/tasks/{id}", post(create_task))
        .route("/tasks/{id}", get(get_task))
        .route("/tasks/{id}", patch(update_task))
        .route("/tasks/{id}", delete(delete_task))
        .route("/tasks/", get(get_all_tasks));

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

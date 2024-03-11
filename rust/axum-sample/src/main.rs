// https://github.com/tokio-rs/axum

use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};
use tokio::signal;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, Root!" }))
        .route("/hello", get(hello))
        .route("/users", post(create_user))
        .route("/user/:id", get(get_user));

    let app = app.fallback(handler_not_found);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn hello() -> &'static str {
    "Hello, World!"
}

// sample fn for json request
// this argument tells axum to parse the request body
// as JSON into a `CreateUser` type
async fn create_user(Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

// sample fn for path parameter
async fn get_user(Path(id): Path<u64>) -> (StatusCode, Json<User>) {
    let user = User {
        id: id.clone(),
        username: "Path".to_string()
    };
    (StatusCode::OK, Json(user))
}

// fn for 404 handling
async fn handler_not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

// the fn for graceful shutdown
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}

# Axum Sample

## Basic

```rust
#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, Root!" }))
        .route("/hello", get(hello))
        .route("/users", post(create_user));

    let app = app.fallback(handler_404);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
```
// https://github.com/tokio-rs/axum

use std::net::SocketAddr;

use axum::{
    async_trait,
    body::{Body, Bytes},
    extract::{FromRequest, Path, Query, Request},
    http::{HeaderValue, Method, StatusCode},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Form, Json, Router,
};
use http_body_util::BodyExt;
use serde::{Deserialize, Serialize};
use tokio::signal;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_consume_body_in_extractor_or_middleware=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let frontend = async {
        let app = Router::new().route("/", get(html));
        serve(app, 3000).await;
    };

    let backend = async {
        // build our application with a single route
        let app = Router::new()
            .route("/", get(|| async { "Hello, Root!" }))
            .route("/hello", get(hello))
            .route("/users", post(create_user))
            .route("/users2", post(create_user2))
            .route("/user/:id", get(get_user))
            .route("/user2", get(get_user2))
            .route("/make-error", get(make_error))
            .route("/json", get(json))
            .layer(middleware::from_fn(print_request_body))
            .layer(
                CorsLayer::new()
                    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                    .allow_methods([Method::GET]),
            );

        let app = app.fallback(handler_not_found);
        serve(app, 4000).await;
    };

    tokio::join!(frontend, backend);
}

// common fn for launching server
async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

// for frontend
async fn html() -> impl IntoResponse {
    Html(
        r#"
        <script>
            fetch('http://localhost:4000/json')
              .then(response => response.json())
              .then(data => console.log(data));
        </script>
        "#,
    )
}

// for frontend
async fn json() -> impl IntoResponse {
    Json(vec!["one", "two", "three"])
}

async fn hello() -> &'static str {
    "Hello, World!"
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

// sample fn for json request
// this argument tells axum to parse the request body
// as JSON into a `CreateUser` type
async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// sample fn for path parameter
async fn get_user(Path(id): Path<u64>) -> (StatusCode, Json<User>) {
    let user = User {
        id: id,
        username: "Path".to_string(),
    };
    (StatusCode::OK, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct GetUser {
    id: u64,
}

// sample fn for query parameter
async fn get_user2(Query(user): Query<GetUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: user.id,
        username: "Query".to_string(),
    };
    (StatusCode::OK, Json(user))
}

// sample handler for form request
async fn create_user2(Form(payload): Form<CreateUser>) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 2222,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

// handler for making error intentionally
// Result<T, E> ... T and E need to be implemented IntoResponse
async fn make_error() -> Result<(), AppError> {
    try_thing()?;
    Ok(())
}

// fn make error
fn try_thing() -> Result<(), anyhow::Error> {
    anyhow::bail!("it failed!")
}

// middleware that shows how to consume the request body upfront
async fn print_request_body(request: Request, next: Next) -> Result<impl IntoResponse, Response> {
    let request = buffer_request_body(request).await?;

    Ok(next.run(request).await)
}

// the trick is to take the request apart, buffer the body, do what you need to do, then put
// the request back together
async fn buffer_request_body(request: Request) -> Result<Request, Response> {
    let (parts, body) = request.into_parts();

    // this wont work if the body is an long running stream
    let bytes = body
        .collect()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?
        .to_bytes();

    do_thing_with_request_body(bytes.clone());

    Ok(Request::from_parts(parts, Body::from(bytes)))
}

fn do_thing_with_request_body(bytes: Bytes) {
    tracing::debug!(body = ?bytes);
}

// extractor that shows how to consume the request body upfront
struct BufferRequestBody(Bytes);

// we must implement `FromRequest` (and not `FromRequestParts`) to consume the body
#[async_trait]
impl<S> FromRequest<S> for BufferRequestBody
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state)
            .await
            .map_err(|err| err.into_response())?;

        do_thing_with_request_body(body.clone());

        Ok(Self(body))
    }
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

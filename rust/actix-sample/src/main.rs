use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// fn for scope
async fn scope() -> impl Responder {
    HttpResponse::Ok().body("scope/index")
}

// This struct represents state
struct AppState {
    app_name: String,
}

// fn for state
#[get("/state")]
async fn state(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

// fn for mutable state
async fn mutable_state(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            // basic
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            // scope
            .service(web::scope("/scope").route("index", web::get().to(scope)))
            // state
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(state)
            .app_data(counter.clone()) // <- register the created data
            .route("/mutable_state", web::get().to(mutable_state))
    })
    .shutdown_timeout(60)
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

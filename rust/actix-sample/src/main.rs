use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// This struct represents state
struct AppState {
    app_name: String,
}

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
async fn index() -> impl Responder {
    HttpResponse::Ok().body("scope/index")
}

// fn for state
#[get("/state")]
async fn state(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // basic
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            // scope
            .service(
                web.scope("/scope")
                .route("index", web::get().to(scope))
            )
            // state
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(state)
    })
    .shutdown_timeout(60)
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
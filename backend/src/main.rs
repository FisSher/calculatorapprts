use actix_web::{ App, HttpResponse, HttpServer, Responder, get, post, web };
use serde::Deserialize;
use actix_cors::Cors;

#[derive(Deserialize)]
struct InputValues {
    value_a: f64,
    value_b: f64,
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
    HttpResponse::Ok().body("Hello! This is a new resource")
}

async fn add(data: web::Json<InputValues>) -> impl Responder {
    HttpResponse::Ok().json(data.value_a + data.value_b)
}

async fn substract(data: web::Json<InputValues>) -> impl Responder {
    HttpResponse::Ok().json(data.value_a - data.value_b)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/add", web::post().to(add))
            .route("/substract", web::post().to(substract))
    })
        .bind(("127.0.0.1", 8080))?
        .run().await
}

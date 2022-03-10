use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

pub mod urlparser;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let x = urlparser::parse_url_local("https://www.rust-lang.org/").unwrap();
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/hey", web::post().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
#[get("/")]
async fn hello() -> impl Responder {
    println!("POOP");
    HttpResponse::Ok().body("Hootie toot!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    println!("POOP");

    HttpResponse::Ok().body("Hey there!")
}

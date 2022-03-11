use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
// use serde::Deserialize;
// use upvotes::VoteEvent;

pub mod upvotes;
pub mod urlparser;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello)
        //.service(echo)
        // .route("/hey", web::get().to(manual_hello))
        // .route("/hey", web::post().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hootie toot!")
}

// #[derive(Deserialize)]
// struct Info {
//     username: String,
// }

// async fn index(info: web::Json<Info>) -> Result<String, String> {
//     Ok(format!("Welcome {}!", info.username))
// }

// #[post("/vote")]
// async fn echo(vote_event: web::Json<VoteEvent>) -> Result<String, String> {
//     Ok(format!("Welcome {}!", vote_event.urlId))
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

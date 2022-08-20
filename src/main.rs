pub mod upvotes;
pub mod urlparser;

use actix_web::{get, web, App, HttpServer, Responder, post};

#[post("/vote")]
async fn echo(vote_event: web::Json<upvotes::VoteEvent>) -> impl Responder {
    let x = &vote_event.url_id;
    format!("Hello {x}!!!")
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!!!")
}


#[post("/hey/{name}")]
async fn greet2(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!!!!!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
            .service(greet2)
            .service(echo)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
// use serde::Deserialize;
// use upvotes::VoteEvent;
// pub mod sqlstuff;
// pub mod upvotes;
// pub mod urlparser;
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().service(hello)
//         // .service(echo)
//         .route("/hey", web::get().to(manual_hello))
//         .route("/hey", web::post().to(manual_hello))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hootie toot!")
// }



// async fn index(info: web::Json<Info>) -> Result<String, String> {
//     Ok(format!("Welcome {}!", info.username))
// }

// // #[post("/vote")]
// // async fn echo(vote_event: web::Json<VoteEvent>) -> Result<String, String> {
// //     Ok(format!("Welcome {}!", vote_event.urlId))
// // }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

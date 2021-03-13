mod endpoints;
#[macro_use]
extern crate serde_derive;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
// use endpoints;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Started");
    HttpServer::new(|| {
        App::new()
            .service(endpoints::hello)
            .service(endpoints::echo)
        // .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:18080")?
    .run()
    .await
}

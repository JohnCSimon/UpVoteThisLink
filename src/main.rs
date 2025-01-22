pub mod common;
pub mod urlparser;

mod endpoints;
use actix_web::{App, HttpServer, web::Data};
use dotenv::dotenv;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = 8086;
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    println!("server started at http://{}:{}", host, port);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(endpoints::utilities::liveness)
            .service(endpoints::urlshortener::do_url_shortening)
            .service(endpoints::voting::do_vote_event)
    })
    .bind((host, port))?
    .run()
    .await
}

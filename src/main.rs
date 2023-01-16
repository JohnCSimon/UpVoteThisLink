pub mod common;
pub mod urlparser;

use actix_web::{web::Data, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv;
mod services;
use services::{do_vote_event};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(do_vote_event)
    })
    .bind(("127.0.0.1", 8086))?
    .run()
    .await
}

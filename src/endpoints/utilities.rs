use actix_web::{HttpResponse, Responder, get};

#[get("/liveness")]
pub async fn liveness() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

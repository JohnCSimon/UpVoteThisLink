use crate::AppState;
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(Deserialize, Serialize, Debug, FromRow)]
pub struct VoteEvent {
    pub url_id: String,
    pub user_id: String,
    // pub event: Event,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Event {
    UpVote,
    DownVote,
}

#[post("/vote")]
async fn do_vote_event(state: Data<AppState>, body: Json<VoteEvent>) -> impl Responder {
    // let x = &vote_event.url_id;
    // format!("Hello {x}!!!")
    match sqlx::query_as::<_, VoteEvent>(
        "INSERT INTO votables (url_id, user_id) VALUES ($1, $2) RETURNING url_id, user_id",
    )
    .bind(body.url_id.to_string())
    .bind(body.user_id.to_string())
    .fetch_one(&state.db)
    .await
    {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string()),
    }
}

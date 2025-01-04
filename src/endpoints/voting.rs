use crate::AppState;
use actix_web::{
    HttpResponse, Responder, get, post,
    web::{Data, Json, Path},
};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct VoteEvent {
    pub url_id: String,
    pub user_id: String,
    pub votetype: i32,
}

#[post("/vote")]
pub async fn do_vote_event(state: Data<AppState>, body: Json<VoteEvent>) -> impl Responder {
    match sqlx::query_as::<_, VoteEvent>(
        "INSERT INTO votables (url_id, user_id, votetype, votedate) VALUES ($1, $2, $3, now()) RETURNING url_id, user_id, votetype, votedate"
    )
    .bind(body.url_id.to_string())
    .bind(body.user_id.to_string())
    .bind(body.votetype)
    .fetch_one(&state.db)
    .await
    {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string()),
    }
}

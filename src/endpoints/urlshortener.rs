use crate::{AppState, urlparser};
use actix_web::{
    HttpResponse, Responder, post,
    web::{Data, Json},
};
use serde::Deserialize;

use sqlx::{self, FromRow};

#[derive(Deserialize)]
pub struct UrlShortenRequest {
    pub url: String,
    pub urlkeys: Vec<String>,
}

// #[derive(Deserialize)]
// pub struct UrlShortenDTO {
//     pub url: String,
//     pub urlkeys: Vec<String>,
// }

// curl --request POST \
//   --url http://127.0.0.1:8086/shortenurl \
//   --header 'Content-Type: application/json' \
//   --header 'User-Agent: insomnia/10.3.0' \
//   --data '{
// 	"url": "http://example.com?v1=t1&v2=t2",
// 	"urlkeys": ["v1"]
// }'

#[post("/shortenurl")]
pub async fn do_url_shortening(
    state: Data<AppState>,
    body: Json<UrlShortenRequest>,
) -> impl Responder {
    let urlkeys: Vec<&str> = body.urlkeys.iter().map(|s| s.as_str()).collect(); // no good!
    let xx = urlparser::remove_query_parameters(&body.url, &urlkeys).unwrap();

    // match sqlx::query_as::<_, UrlShortenDTO>(
    //     "INSERT INTO votables (url_id, user_id, votetype, votedate) VALUES ($1, $2, $3, now()) RETURNING url_id, user_id, votetype, votedate"5
    // )
    // .bind(body.url_id.to_string())
    // .bind(body.user_id.to_string())
    // .bind(body.votetype)
    // .fetch_one(&state.db)
    // .await
    // {
    //     Ok(article) => HttpResponse::Ok().json(article),
    //     Err(error) => HttpResponse::InternalServerError().json(error.to_string()),
    // }
    HttpResponse::Ok().json(xx)
}

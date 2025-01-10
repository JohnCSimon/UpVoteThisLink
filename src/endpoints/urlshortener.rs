use crate::{AppState, urlparser::remove_query_parameters};
use actix_web::{
    HttpResponse, Responder, post,
    web::{Data, Json},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{self, FromRow};

#[derive(Deserialize)]
pub struct UrlShortenRequest {
    pub url: String,
    pub urlkeys: Vec<String>,
}

#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct UrlShortenDTO {
    pub url: String,
    pub urlhash: String,
}

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
    let url = remove_query_parameters(&body.url, &urlkeys).unwrap();

    // Create a Sha256 hasher instance
    let mut hasher = Sha256::new();
    hasher.update(url.clone());

    // Finalize the hash and truncate it
    let hash = hasher.finalize();
    let truncated_hash = &hash[..8]; // Take the first 8 bytes

    // Convert to hexadecimal
    let hash_hex = format!(
        "{:x}",
        truncated_hash
            .iter()
            .fold(0, |acc, &b| (acc << 8) | b as u64)
    );

    let insert_row: Result<UrlShortenDTO, sqlx::Error> = sqlx::query_as::<_, UrlShortenDTO>(
        "INSERT INTO hashedurls (url, hash) VALUES ($1, $2, now()) RETURNING hash",
    )
    .bind(url.to_string())
    .bind(hash_hex.to_string())
    .fetch_one(&state.db)
    .await;

    match insert_row {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(error) => HttpResponse::InternalServerError().json(error.to_string()),
    }
}
// let urlkeys: Vec<&str> = body.urlkeys.iter().map(|s| s.as_str()).collect(); // no good!
// let url = urlparser::remove_query_parameters(&body.url, &urlkeys).unwrap();

// // Create a Sha256 hasher instance
// let mut hasher = Sha256::new();
// hasher.update(url);

// // Finalize the hash and truncate it
// let hash = hasher.finalize();
// let truncated_hash = &hash[..8]; // Take the first 8 bytes

// // Convert to hexadecimal
// let hash_hex = format!(
//     "{:x}",
//     truncated_hash
//         .iter()
//         .fold(0, |acc, &b| (acc << 8) | b as u64)
// );

// let insert_row: UrlShortenDTO = match sqlx::query_as::<_, UrlShortenDTO>(
//     "INSERT INTO hashedurls (url, hash) VALUES ($1, $2, now()) RETURNING hash",
// )
// .bind(url.to_string())
// .bind(hash_hex.to_string())
// .fetch_one(&state.db)
// .await
// {
//     Ok(article) => HttpResponse::Ok().json(article),
//     Err(error) => HttpResponse::InternalServerError().json(error.to_string()),
// }

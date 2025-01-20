use crate::{AppState, urlparser::remove_query_parameters};
use actix_web::{
    HttpResponse, Responder, post,
    web::{Data, Json},
};
use base64;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
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

fn generate_truncated_hash(url: &str) -> String {
    // Create a Sha256 hasher instance
    let mut hasher = Sha256::new();
    hasher.update(url);

    // Finalize the hash and truncate it
    let hash = hasher.finalize();

    // encode hash as a base64 encoded string slice
    URL_SAFE_NO_PAD.encode(hash)
}

#[post("/shortenurl")]
pub async fn do_url_shortening(
    state: Data<AppState>,
    body: Json<UrlShortenRequest>,
) -> impl Responder {
    let urlkeys: Vec<&str> = body.urlkeys.iter().map(|s| s.as_str()).collect(); // no good!
    let url = remove_query_parameters(&body.url, &urlkeys).unwrap();

    let hash_hex = generate_truncated_hash(&url);

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

#[cfg(test)]
mod tests {

    #[test]
    fn test_generate_truncated_hash_basic() {
        let url = "https://example.com";
        let result = crate::endpoints::urlshortener::generate_truncated_hash(url);

        // Ensure the output is not empty
        assert!(!result.is_empty(), "Hash should not be empty");

        // Ensure the output contains only URL-safe characters
        assert!(
            result
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
            "Hash contains non-URL-safe characters"
        );
    }

    #[test]
    fn test_generate_truncated_hash_empty_string() {
        let url = "";
        let result = crate::endpoints::urlshortener::generate_truncated_hash(url);

        // Ensure the output is still valid
        assert!(
            !result.is_empty(),
            "Hash should not be empty for an empty string"
        );

        // Ensure the output contains only URL-safe characters
        assert!(
            result
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
            "Hash contains non-URL-safe characters"
        );
    }

    #[test]
    fn test_generate_truncated_hash_different_inputs() {
        let url1 = "https://example.com";
        let url2 = "https://example.org";

        let result1 = crate::endpoints::urlshortener::generate_truncated_hash(url1);
        let result2 = crate::endpoints::urlshortener::generate_truncated_hash(url2);

        // Ensure different inputs produce different outputs
        assert_ne!(
            result1, result2,
            "Different inputs should produce different hashes"
        );
    }

    #[test]
    fn test_generate_truncated_hash_repeatable() {
        let url = "https://example.com";

        let result1 = crate::endpoints::urlshortener::generate_truncated_hash(url);
        let result2 = crate::endpoints::urlshortener::generate_truncated_hash(url);

        // Ensure the same input produces the same output
        assert_eq!(result1, result2, "Hash generation should be repeatable");
    }

    #[test]
    fn test_generate_truncated_hash_truncation() {
        let url = "https://example.com";
        let result = crate::endpoints::urlshortener::generate_truncated_hash(url);

        // Ensure the hash is truncated to the expected length (modify this if truncation is added)
        // Note: Current implementation doesn't truncate, so check the full hash length.
        assert_eq!(
            result.len(),
            43,
            "Expected hash length is 43 for full SHA256 encoded in URL-safe Base64"
        );
    }
}

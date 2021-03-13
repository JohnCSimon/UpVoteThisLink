use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike, Utc};

use actix_web::App;
use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, HttpServer, Responder,
};

#[derive(Deserialize)]
pub struct MyInput {
    username: String,
}

impl MyInput {
    fn getTime(&self) -> String {
        let now = Utc::now();
        let (is_common_era, year) = now.hour12();

        let y = format!(
            "{}, The current UTC date is {}-{:02}-{:02} {:?}",
            self.username,
            year,
            now.month(),
            now.day(),
            now.weekday()
        );
        return y;
    }
}

#[get("/")]
pub async fn hello(infoo: Json<MyInput>) -> impl Responder {
    let _x = MyInput::getTime(&infoo.0);
    HttpResponse::Ok().body("Hello world!".to_string() + &infoo.username + &_x)
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

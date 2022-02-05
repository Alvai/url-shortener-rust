use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    url: String,
    name: String,
}

pub async fn encode_link(info: web::Json<Info>) -> impl Responder {
    println!("link: {} from {}", info.url, info.name);
    HttpResponse::Ok()
}

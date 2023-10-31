//! src/routes/login/post.rs
//! https://developer.mozilla.org/en-US/docs/Web/HTTP/Redirections

use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;

pub async fn login() -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, "/"))
        .finish()
}

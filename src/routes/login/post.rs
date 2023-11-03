//! src/routes/login/post.rs
//! https://developer.mozilla.org/en-US/docs/Web/HTTP/Redirections

use crate::authentication::AuthError;
use crate::routes::error_chain_fmt;
use actix_web::{
    error::InternalError,
    http::{
        header::{ContentType, LOCATION},
        StatusCode,
    },
    web, HttpResponse, ResponseError,
};
use actix_web_flash_messages::FlashMessage;
use secrecy::Secret;
use sqlx::PgPool;

use crate::authentication::{validate_credentials, Credentials};

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: Secret<String>,
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

// impl ResponseError for LoginError {
//     fn status_code(&self) -> StatusCode {
//         match self {
//             LoginError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
//             LoginError::AuthError(_) => StatusCode::UNAUTHORIZED,
//         }
//     }

//     fn error_response(&self) -> HttpResponse {
//         let query_string = format!("error={}", urlencoding::Encoded::new(self.to_string()));
//         // We need the secret here - how do we get it?
//         let secret: &[u8] = todo!();
//         let hmac_tag = {
//             let mut mac = Hmac::<sha2::Sha256>::new_from_slice(secret).unwrap();
//             mac.update(query_string.as_bytes());
//             mac.finalize().into_bytes()
//         };
//         // let encoded_error = urlencoding::Encoded::new(self.to_string());
//         HttpResponse::build(self.status_code())
//             .insert_header((LOCATION, format!("/login?{query_string}&tag={hmac_tag:x}")))
//             .finish()
//     }
// }

#[tracing::instrument(skip(form, pool), fields(username=tracing::field::Empty, user_id=tracing::field::Empty))]
pub async fn login(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = Credentials {
        username: form.0.username,
        password: form.0.password,
    };
    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));
    match validate_credentials(credentials, &pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/"))
                .finish())
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };
            // Creates, signs (with hmac), sets correct properties of cookie
            FlashMessage::error(e.to_string()).send();
            let response = HttpResponse::SeeOther()
                .insert_header((LOCATION, "/login"))
                .insert_header(("Set-Cookie", format!("_flash={e}")))
                .finish();
            Err(InternalError::from_response(e, response))
        }
    }
}
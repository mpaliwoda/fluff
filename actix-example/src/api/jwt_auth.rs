use actix_web::HttpMessage;
use std::{
    fmt,
    future::{ready, Ready},
};

use actix_web::{
    dev::Payload, error::ErrorUnauthorized, http::header, web, Error as ActixWebError, FromRequest, HttpRequest,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use uuid::Uuid;

use crate::{users::models::TokenClaims, AppContext};

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = serde_json::to_string(&self).expect("Failed to serialize ErrorResponse");
        write!(f, "{}", repr)
    }
}

pub struct JwtMiddleware {
    pub user_id: Uuid,
}

impl FromRequest for JwtMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let ctx = req
            .app_data::<web::Data<AppContext>>()
            .expect("Failed to obtain app context");

        let token = req
            .cookie("token")
            .map(|cookie| cookie.value().to_string())
            .or_else(|| token_from_header(req));

        if token.is_none() {
            return unauthorized_response("No bearer token provied.");
        }

        let token = token.unwrap();
        let decoding_key = DecodingKey::from_secret(ctx.config.jwt_secret.as_ref());

        let claims = match decode::<TokenClaims>(&token, &decoding_key, &Validation::default()) {
            Ok(c) => c.claims,
            Err(_) => return unauthorized_response("Invalid bearer token provided."),
        };

        let user_id = Uuid::parse_str(claims.sub.as_str()).expect("Failed parsing UUID.");

        req.extensions_mut().insert::<uuid::Uuid>(user_id.to_owned());

        ready(Ok(JwtMiddleware { user_id }))
    }
}

fn unauthorized_response(message: &str) -> Ready<Result<JwtMiddleware, ActixWebError>> {
    let err = ErrorResponse {
        status: "fail".into(),
        message: String::from(message),
    };

    ready(Err(ErrorUnauthorized(err)))
}

fn token_from_header(req: &HttpRequest) -> Option<String> {
    let (auth_type, secret) = req
        .headers()
        .get(header::AUTHORIZATION)?
        .to_str()
        .ok()?
        .split_once(" ")?;

    if auth_type != "Bearer" {
        return None;
    }

    Some(String::from(secret))
}

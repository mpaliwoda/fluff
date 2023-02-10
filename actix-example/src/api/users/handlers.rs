use crate::api::jwt_auth::JwtMiddleware;
use crate::users::repository as users_repository;
use crate::AppContext;
use crate::{api::users::schemas::responses::FilteredUser, users::models::TokenClaims};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

use super::schemas::requests::{LoginUserSchema, RegisterUserSchema};

const USER_EXISTS: &'static str = "user with provided email already exists";
const INVALID_CREDENTIALS: &'static str = "invalid credentials provied";

#[post("/register")]
async fn register_user_handler(body: web::Json<RegisterUserSchema>, ctx: web::Data<AppContext>) -> impl Responder {
    match users_repository::exists(body.email.to_owned(), &ctx.db_pool)
        .await
        .expect("Failed to check if user exists")
    {
        true => return HttpResponse::Conflict().json(json!({ "error": USER_EXISTS })),
        false => (),
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Failed to encrypt password");

    let create_user_result = users_repository::create(
        body.name.to_string(),
        body.email.to_string(),
        hashed_password.to_string(),
        body.role.to_owned(),
        &ctx.db_pool,
    )
    .await;

    match create_user_result {
        Ok(user) => HttpResponse::Ok().json(json!(FilteredUser::from_db_user(&user))),
        Err(e) => HttpResponse::InternalServerError().json(json!({ "error": e })),
    }
}

#[post("/login")]
async fn login_user_handler(body: web::Json<LoginUserSchema>, ctx: web::Data<AppContext>) -> impl Responder {
    let user = users_repository::get_user_by_email(body.email.to_owned(), &ctx.db_pool)
        .await
        .expect("Failed to get user from db");

    let is_valid = user.to_owned().map_or(false, |user| {
        let parsed_hash = PasswordHash::new(&user.password).expect("Failed to generate password hash");
        Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true)
    });

    if !is_valid {
        return HttpResponse::BadRequest().json(json!({ "error": INVALID_CREDENTIALS }));
    }

    let user = user.unwrap();
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(ctx.config.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success", "token": token}))
}

#[get("/logout")]
async fn logout_handler(_: JwtMiddleware) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok().cookie(cookie).json(json!({"status": "success"}))
}

#[get("/whoami")]
async fn whoami_handler(req: HttpRequest, ctx: web::Data<AppContext>, _: JwtMiddleware) -> impl Responder {
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    let user = users_repository::get_user_by_id(user_id, &ctx.db_pool)
        .await
        .expect("Failed to fetch current user by id");

    let json_response = serde_json::json!(FilteredUser::from_db_user(&user));

    HttpResponse::Ok().json(json_response)
}

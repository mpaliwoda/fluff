mod api;
mod config;
mod users;

use actix_cors::Cors;
use actix_web::{
    middleware::{Compress, Logger},
    web, App, HttpServer,
};
use api::users::handlers::{login_user_handler, logout_handler, register_user_handler, whoami_handler};
use config::Config;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub struct AppContext {
    pub config: Config,
    pub db_pool: PgPool,
}

fn setup_dev() {
    if std::env::var("ENVIRONMENT").unwrap_or("development".into()) == "development" {
        use dotenv::dotenv;
        dotenv().ok();
    }
}

fn setup_logging() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    env_logger::init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // if `ENVIRONMENT` var is set and not equal to `development` this is a noop
    // otherwise, it'll source contents of .env file
    setup_dev();

    // set up env_logger
    setup_logging();

    // create shared config with env vars and db_pool
    let config = Config::new();
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");


    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(Cors::default().supports_credentials())
            .app_data(web::Data::new(AppContext {
                db_pool: db_pool.clone(),
                config: config.clone(),
            }))
            .service(api::health::healthcheck)
            .service(
                web::scope("/auth")
                    .service(register_user_handler)
                    .service(login_user_handler)
                    .service(logout_handler)
                    .service(whoami_handler),
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

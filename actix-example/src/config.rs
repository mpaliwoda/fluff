use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: isize,
}

impl Config {
    pub fn new() -> Config {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not provided");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not provided");
        let jwt_expires_in = env::var("JWT_EXPIRES_IN").expect("JWT_EXPIRES_IN not provided");
        let jwt_maxage = env::var("JWT_MAXAGE")
            .expect("JWT_MAXAGE not provided")
            .parse()
            .expect("JWT_MAXAGE is not an integer");

        Config {
            database_url,
            jwt_secret,
            jwt_expires_in,
            jwt_maxage,
        }
    }
}

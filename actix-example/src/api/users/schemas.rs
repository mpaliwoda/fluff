pub mod requests {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct RegisterUserSchema {
        pub name: String,
        pub email: String,
        pub password: String,
        pub role: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    pub struct LoginUserSchema {
        pub email: String,
        pub password: String,
    }
}

pub mod responses {
    use chrono::prelude::*;
    use serde::Serialize;

    use crate::users::models::User;

    #[allow(non_snake_case)]
    #[derive(Debug, Serialize)]
    pub struct FilteredUser {
        pub id: String,
        pub name: String,
        pub email: String,
        pub role: String,
        pub verified: bool,
        pub createdAt: DateTime<Utc>,
        pub updatedAt: DateTime<Utc>,
    }

    impl FilteredUser {
        pub fn from_db_user(user: &User) -> FilteredUser {
            FilteredUser {
                id: user.id.to_string(),
                email: user.email.to_owned(),
                name: user.name.to_owned(),
                role: user.role.to_owned(),
                verified: user.verified,
                createdAt: user.created_at.unwrap(),
                updatedAt: user.updated_at.unwrap(),
            }
        }
    }

    #[derive(Debug, Serialize)]
    pub struct UserData {
        pub user: FilteredUser,
    }

    #[derive(Debug, Serialize)]
    pub struct UserResponse {
        pub status: String,
        pub data: UserData,
    }
}

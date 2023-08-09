use std::time::Duration;

use poem::{
    error::InternalServerError,
    http::StatusCode,
    web::{
        cookie::{Cookie, CookieJar, SameSite},
        Data,
    },
    Request,
};
use poem_openapi::{payload::Json, types::Email, Object, OpenApi, SecurityScheme};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::{
    database::Database,
    models::{self, NewUser},
    schema::{self, users},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

pub struct Auth;

#[derive(SecurityScheme)]
#[oai(ty = "bearer", checker = "api_checker")]
struct AppAuthorization(String);

async fn api_checker(_req: &Request, bearer: poem_openapi::auth::Bearer) -> Option<String> {
    Some(bearer.token)
}

#[derive(Debug, Object)]
struct LoginRequest {
    /// The name of the user
    username: String,

    /// Password
    password: String,
}

#[derive(Debug, Object)]
struct RegisterRequest {
    /// The name of the user
    username: String,

    /// The email of the user
    email: Email,

    /// Password
    password: String,
}

#[derive(Debug, Object)]
struct User {
    /// The name of the user
    username: String,
}

// TODO: https://docs.rs/argon2/latest/argon2/

#[OpenApi]
impl Auth {
    /// Login the user
    #[oai(path = "/register", method = "post", operation_id = "register")]
    async fn register(
        &self,
        req: Json<RegisterRequest>,
        database: Data<&Database>,
    ) -> poem::Result<Json<User>> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = argon2
            .hash_password(req.password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        let new_user = NewUser {
            email: &req.email,
            username: &req.username,
            password_hash: &password_hash,
        };
        let inserted = diesel::insert_into(users::table)
            .values(&new_user)
            .returning(models::User::as_returning())
            .get_result(&mut database.get().await.map_err(InternalServerError)?)
            .await
            .expect("Error saving new user");

        Ok(Json(User {
            username: inserted.username,
        }))
    }

    /// Login the user
    #[oai(path = "/login", method = "post", operation_id = "login")]
    async fn login(
        &self,
        req: Json<LoginRequest>,
        cookie_jar: &CookieJar,
        database: Data<&Database>,
    ) -> poem::Result<Json<User>> {
        use schema::users::dsl::*;

        let Ok(user) = users
            .select(models::User::as_select())
            .filter(username.eq(&req.username).or(email.eq(&req.username)))
            .first(&mut database.get().await.map_err(InternalServerError)?)
            .await
        else {
            return Err(poem::Error::new(
                poem_openapi::error::AuthorizationError,
                StatusCode::UNAUTHORIZED,
            ));
        };

        let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();

        if Argon2::default()
            .verify_password(req.password.as_bytes(), &parsed_hash)
            .is_err()
        {
            return Err(poem::Error::new(
                poem_openapi::error::AuthorizationError,
                StatusCode::UNAUTHORIZED,
            ));
        }

        // TODO: JWT
        let mut cookie = Cookie::new_with_str("Authorization", &req.username);
        cookie.set_http_only(true);
        cookie.set_secure(true);
        cookie.set_same_site(Some(SameSite::Strict));
        cookie.set_max_age(Duration::from_secs(3600 * 24 * 365));
        cookie_jar.add(cookie);

        Ok(Json(User {
            username: req.0.username,
        }))
    }

    /// Return the currently logged in user
    #[oai(path = "/me", method = "post", operation_id = "me")]
    async fn me(&self, auth: AppAuthorization) -> Json<User> {
        Json(User { username: auth.0 })
    }

    /// Logout the user
    #[oai(path = "/logout", method = "post", operation_id = "logout")]
    async fn logout(&self, cookie_jar: &CookieJar) {
        // Clear authentification cookie by setting a duration of 0.
        // Using cookie_jar.remove(...) causes Firefox to complain about SameSite.
        let mut cookie = Cookie::new_with_str("Authorization", "");
        cookie.set_http_only(true);
        cookie.set_secure(true);
        cookie.set_same_site(Some(SameSite::Strict));
        cookie.set_max_age(Duration::from_secs(0));
        cookie_jar.add(cookie);
    }
}

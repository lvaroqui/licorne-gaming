use std::time::Duration;

use bearer_from_cookie_to_header::BearerFromCookieToHeader;
use poem::{
    listener::TcpListener,
    middleware::CookieJarManager,
    web::cookie::{Cookie, CookieJar},
    EndpointExt, Request, Route,
};
use poem_openapi::{
    param::Query, payload::Json, types::Example, Object, OpenApi, OpenApiService, SecurityScheme,
};

mod bearer_from_cookie_to_header;

struct Api;

#[derive(SecurityScheme)]
#[oai(ty = "bearer", checker = "api_checker")]
struct AppAuthorization(String);

async fn api_checker(_req: &Request, bearer: poem_openapi::auth::Bearer) -> Option<String> {
    if &bearer.token == "tototiti" {
        Some(bearer.token)
    } else {
        None
    }
}

#[derive(Debug, Object)]
#[oai(example)]
struct HelloResponse {
    /// The name of the user
    name: String,
}

impl Example for HelloResponse {
    fn example() -> Self {
        Self {
            name: "Luc".to_string(),
        }
    }
}

#[OpenApi]
impl Api {
    /// Retrieve the name of the user
    #[oai(path = "/hello", method = "get", operation_id = "hello")]
    async fn index(&self, name: Query<String>) -> Json<HelloResponse> {
        Json(HelloResponse {
            name: "Hello ".to_string() + &name.0,
        })
    }

    /// Retrieve the name of the user
    #[oai(path = "/hello_protected", method = "get")]
    async fn index_protected(
        &self,
        name: Query<String>,
        auth: AppAuthorization,
    ) -> Json<HelloResponse> {
        Json(HelloResponse {
            name: "Hello ".to_string() + &name.0 + &auth.0,
        })
    }

    /// Retrieve the name of the user
    #[oai(path = "/login", method = "get")]
    async fn login(&self, cookie_jar: &CookieJar) {
        let mut cookie = Cookie::new_with_str("Bearer", "tototiti");
        cookie.set_http_only(true);
        cookie.set_secure(true);
        cookie.set_max_age(Duration::from_secs(3600 * 24 * 365));
        cookie_jar.add(cookie);
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service = OpenApiService::new(Api, "Hello World", "1.0.0").server("/api");
    let ui = api_service.swagger_ui();
    let spec = api_service.spec_endpoint();
    let app = Route::new()
        .nest("/api", api_service)
        .nest("/api/docs", ui)
        .nest("/api/spec", spec)
        .with(BearerFromCookieToHeader)
        .with(CookieJarManager::new());

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}

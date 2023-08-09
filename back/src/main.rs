use bearer_from_cookie_to_header::BearerFromCookieToHeader;
use database::init_pool;
use poem::{listener::TcpListener, middleware::CookieJarManager, EndpointExt, Route};
use poem_openapi::OpenApiService;

mod auth;
mod bearer_from_cookie_to_header;
mod database;
pub mod models;
pub mod schema;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let db_pool = init_pool();

    let api = OpenApiService::new(auth::Auth, "Licorne Gaming", "1.0.0").server("/api");
    let docs = api.swagger_ui();
    let spec = api.spec_endpoint();

    let app = Route::new()
        .nest("/api", api)
        .nest("/api/docs", docs)
        .nest("/api/spec", spec)
        .data(db_pool)
        .with(BearerFromCookieToHeader)
        .with(CookieJarManager::new());

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}

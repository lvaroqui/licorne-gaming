use poem::{listener::TcpListener, Request, Route};
use poem_openapi::{
    auth::ApiKey, param::Query, payload::Json, types::Example, Object, OpenApi, OpenApiService,
    SecurityScheme,
};

struct Api;

#[derive(SecurityScheme)]
#[oai(
    ty = "api_key",
    key_name = "X-API-Key",
    key_in = "header",
    checker = "api_checker"
)]
struct AppAuthorization(String);

async fn api_checker(_req: &Request, _api_key: ApiKey) -> Option<String> {
    Some("toto".to_string())
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
        _auth: AppAuthorization,
    ) -> Json<HelloResponse> {
        Json(HelloResponse {
            name: "Hello ".to_string() + &name.0,
        })
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
        .nest("/api/spec", spec);

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}

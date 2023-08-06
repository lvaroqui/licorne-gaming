use poem::{listener::TcpListener, Route};
use poem_openapi::{param::Query, payload::Json, types::Example, Object, OpenApi, OpenApiService};

struct Api;

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
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<String>) -> Json<HelloResponse> {
        Json(HelloResponse {
            name: "Hello ".to_string() + &name.0,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/api/docs", ui);

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}

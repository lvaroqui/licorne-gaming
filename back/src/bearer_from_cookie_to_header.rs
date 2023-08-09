use poem::{http::HeaderValue, Endpoint, Middleware, Request, Result};

/// A middleware that extract Bearer token from Secure Cookie and adds it as
/// Authorization header (so that is is properly grabbed by OpenAPI Bearer extractor).
pub struct BearerFromCookieToHeader;

impl<E: Endpoint> Middleware<E> for BearerFromCookieToHeader {
    type Output = TokenMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        TokenMiddlewareImpl { ep }
    }
}

pub struct TokenMiddlewareImpl<E> {
    ep: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for TokenMiddlewareImpl<E> {
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        if let Some(bearer) = req.cookie().get("Authorization") {
            if let Ok(header_value) =
                HeaderValue::from_str(&format!("Bearer {}", bearer.value_str()))
            {
                req.headers_mut().insert("Authorization", header_value);
            }
        }
        self.ep.call(req).await
    }
}

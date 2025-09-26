use axum::{
    extract::Request,
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};

pub fn create_auth_middleware(
    required_token: String,
) -> impl Fn(
    Request,
    Next,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Response, StatusCode>> + Send>,
> + Clone {
    move |req: Request, next: Next| {
        let required_token = required_token.clone();
        Box::pin(async move {
            let auth_header = req
                .headers()
                .get(AUTHORIZATION)
                .and_then(|header| header.to_str().ok());

            let token = if let Some(auth_header) = auth_header {
                if auth_header.starts_with("Bearer ") {
                    Some(auth_header.trim_start_matches("Bearer "))
                } else {
                    None
                }
            } else {
                None
            };

            match token {
                Some(provided_token) if provided_token == required_token => Ok(next.run(req).await),
                _ => Err(StatusCode::UNAUTHORIZED),
            }
        })
    }
}

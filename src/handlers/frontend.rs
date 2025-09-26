use axum::{
    http::{header, StatusCode},
    response::{Html, Response},
};
use std::fs;

pub async fn serve_upload_page() -> Result<Html<String>, StatusCode> {
    let html_content =
        fs::read_to_string("static/upload.html").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Html(html_content))
}

pub async fn serve_style_css() -> Result<Response, StatusCode> {
    let css_content =
        fs::read_to_string("static/style.css").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/css")
        .header(header::CACHE_CONTROL, "public, max-age=3600")
        .body(css_content.into())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
}

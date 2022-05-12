use super::web_error::WebError;

use actix_web::{HttpResponse, HttpResponseBuilder};
use serde::Serialize;

const DEFAULT_HEADER: (&str, &str) = ("Content-Type", "text/json;charset=UTF-8");

pub fn handle_success<Model: Serialize>(
    mut builder: HttpResponseBuilder,
    data: Model,
) -> HttpResponse {
    builder
        .insert_header(DEFAULT_HEADER)
        .body(serde_json::json!(data).to_string())
}

pub fn handle_error(err: WebError) -> HttpResponse {
    HttpResponse::build(err.code)
        .insert_header(DEFAULT_HEADER)
        .body(serde_json::json!(err.format()).to_string())
}

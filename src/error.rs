use std::fmt;

use axum::http::StatusCode;

pub type ResponseError = (StatusCode, String);

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: Into<eyre::Report> + fmt::Display,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}


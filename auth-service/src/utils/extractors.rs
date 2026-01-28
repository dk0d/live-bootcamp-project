use axum::extract::{FromRequest, Request};
use axum::response::IntoResponse;
use axum::{Form, Json};
use serde::de::DeserializeOwned;

#[derive(Debug, PartialEq, Eq)]
pub enum FormOrJsonError {
    Invalid,
    Form(String),
    Json(String),
}

#[derive(Debug, Clone, Copy)]
pub struct FormOrJson<T>(pub T);

impl IntoResponse for FormOrJsonError {
    fn into_response(self) -> axum::response::Response {
        match self {
            FormOrJsonError::Invalid => {
                let body = serde_json::json!({
                    "error": "Invalid request format"
                });
                (
                    axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                    axum::Json(body),
                )
                    .into_response()
            }
            FormOrJsonError::Form(err_msg) => {
                let body = serde_json::json!({
                    "error": format!("Form parsing error: {}", err_msg)
                });
                (
                    axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                    axum::Json(body),
                )
                    .into_response()
            }
            FormOrJsonError::Json(err_msg) => {
                let body = serde_json::json!({
                    "error": format!("JSON parsing error: {}", err_msg)
                });
                (
                    axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                    axum::Json(body),
                )
                    .into_response()
            }
        }
    }
}

impl<T, S> FromRequest<S> for FormOrJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = FormOrJsonError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match req
            .headers()
            .get(axum::http::header::CONTENT_TYPE)
            .map(|c| c.to_str().unwrap_or(""))
        {
            Some(content_type) if content_type.starts_with("application/json") => {
                // Try to extract as JSON
                match Json::<T>::from_request(req, state).await {
                    Ok(Json(value)) => return Ok(FormOrJson(value)),
                    Err(err) => return Err(FormOrJsonError::Json(err.to_string())),
                }
            }
            Some(content_type) if content_type.starts_with("application/x-www-form-urlencoded") => {
                // Try to extract as Form
                match Form::<T>::from_request(req, state).await {
                    Ok(Form(value)) => return Ok(FormOrJson(value)),
                    Err(err) => return Err(FormOrJsonError::Form(err.to_string())),
                }
            }
            _ => {}
        }

        // If both extractions fail, return an error
        Err(FormOrJsonError::Invalid)
    }
}

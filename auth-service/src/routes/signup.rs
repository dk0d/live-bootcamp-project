use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use tracing::instrument;
use utoipa::ToSchema;

fn default_false() -> bool {
    false
}

#[derive(serde::Deserialize, Serialize, Debug, ToSchema)]
#[serde(tag = "method", rename_all = "snake_case")]
pub enum SignupRequest {
    /// Signup using email and password
    #[schema(title = "Email/Password")]
    EmailPassword {
        email: String,
        password: String,
        #[serde(default = "default_false")]
        requires_2fa: bool,
    },

    /// Signup using magic link sent to email
    ///
    /// Coming soon...
    #[schema(title = "Magic Link")]
    MagicLink { email: String },

    /// Signup using passkey (WebAuthn)
    ///
    /// Coming soon...
    #[schema(title = "Passkey/WebAuthn")]
    Passkey { email: String },
}

#[utoipa::path(
    post,
    path = "/signup",
    tag = "Authentication",
    responses(
        (status = 200, description = "Signup successful"),
        (status = 400, description = "Bad Request")
    )
)]
#[instrument]
pub async fn signup_handler(Json(body): Json<SignupRequest>) -> impl IntoResponse {
    // Placeholder for signup logic
    (StatusCode::OK, "Signup successful").into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signup_request_schema() {
        let req = SignupRequest::EmailPassword {
            email: "testuser@hello.com".to_string(),
            password: "password123".to_string(),
            requires_2fa: false,
        };
        let schema = serde_json::to_string_pretty(&req).unwrap();
        println!("SignupRequest Schema: {}", schema);
        assert!(schema.contains("method"));
    }
}

use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

mod health;
mod login;
mod logout;
mod signup;
mod verify_2fa;
mod verify_token;

pub use health::*;
pub use login::*;
pub use logout::*;
pub use signup::*;
pub use verify_2fa::*;
pub use verify_token::*;

use crate::openapi::ApiDoc;
use crate::state::AppState;

pub fn build_app_router(state: AppState) -> OpenApiRouter {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(root))
        .routes(routes!(hello_handler))
        .routes(routes!(healthz))
        .routes(routes!(livez))
        .routes(routes!(login_handler))
        .routes(routes!(signup_handler))
        .routes(routes!(logout_handler))
        .routes(routes!(verify_2fa_handler))
        .routes(routes!(verify_token_handler))
        .routes(routes!(readyz))
        .with_state(state)
}

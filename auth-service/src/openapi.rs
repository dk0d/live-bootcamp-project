use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // components are available when using `#[derive(OpenApi)]`
        let components = openapi.components.as_mut().unwrap();

        components.add_security_scheme(
            "AuthorizationPayload",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::with_description(
                "AuthorizationPayload",
                "Authorization payload containing the base64-encoded JWT token",
            ))),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Authx API",
        // read from Cargo.toml .version
        version = env!("CARGO_PKG_VERSION"),
        description = "Authx is a composable authentication and authorization server built with Rust.",
        contact(
            name = "Daniel Capecci",
            url = "https://authxrs.io"
        ),
        license(
            name = "Proprietary"
        )
    ),
    modifiers(&SecurityAddon),
    servers(
       (url = "/", description = "Local"),
    ),
    security(
        ("AuthorizationPayload" = []),
    )
)]
pub struct ApiDoc;

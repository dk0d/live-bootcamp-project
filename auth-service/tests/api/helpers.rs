#![allow(dead_code)]

use axum_test::TestResponse;
use libauth_service::config::Config;
use libauth_service::Application;

pub struct TestServer {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestServer {
    pub async fn new(config: &Config) -> Self {
        let app = Application::build(config)
            .await
            .expect("Failed to build application.");

        let address = format!("http://{}", app.address.clone());

        let http_client = reqwest::Client::builder()
            .user_agent("auth-service-test")
            .build()
            .expect("Failed to build HTTP client.");

        // Run server in background task
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        Self {
            address,
            http_client,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&self.address)
            .send()
            .await
            .expect("Failed to send GET request to root.")
    }

    pub async fn get_livez(&self) -> reqwest::Response {
        self.http_client
            .get(format!("{}/livez", &self.address))
            .send()
            .await
            .expect("Failed to send GET request to /livez.")
    }
}

/// Alternative TestApp using `axum_test` for making requests directly to the app router
/// without networking.
///
/// benefits (?)
/// - faster tests?
/// - doesn't require binding to ports - doesn't need to worry about port conflicts
/// - easier path manipulation?
/// - ...
pub struct TestApp {
    pub server: axum_test::TestServer,
}

impl TestApp {
    pub async fn new(config: &Config) -> Self {
        let app = Application::build_router(config)
            .await
            .expect("Failed to build application.");
        let server = axum_test::TestServer::new(app).expect("Failed to start test server.");
        Self { server }
    }

    pub async fn get_root(&self) -> TestResponse {
        self.server.get("/").await
    }

    pub async fn get_livez(&self) -> TestResponse {
        self.server.get("/livez").await
    }
}

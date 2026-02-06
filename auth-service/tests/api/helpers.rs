#![allow(dead_code)]

use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use axum_test::{TestRequest, TestResponse};
use lgr_auth::Application;
use lgr_auth::config::Config;
use tokio::sync::OnceCell;

static APP: OnceCell<TestApp> = OnceCell::const_new();

pub async fn get_test_app() -> &'static TestApp {
    APP.get_or_init(|| async { TestApp::new(&Config::default()).await })
        .await
}

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
            .user_agent("lgr_auth-test")
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

    pub async fn post_login(&self) -> impl IntoResponse {
        self.http_client
            .post(format!("{}/login", &self.address))
            .send()
            .await
            .expect("Failed to send POST request to /login.");
    }

    pub async fn post_signup(&self) -> impl IntoResponse {
        self.http_client
            .post(format!("{}/signup", &self.address))
            .send()
            .await
            .expect("Failed to send POST request to /signup.");
    }

    pub async fn post_logout(&self) -> impl IntoResponse {
        self.http_client
            .post(format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to send POST request to /logout.");
    }

    pub async fn post_verify_2fa(&self) -> impl IntoResponse {
        self.http_client
            .post(format!("{}/verify-2fa", &self.address))
            .send()
            .await
            .expect("Failed to send POST request to /verify-2fa.");
    }

    pub async fn post_verify_token(&self) -> impl IntoResponse {
        self.http_client
            .post(format!("{}/verify-token", &self.address))
            .send()
            .await
            .expect("Failed to send POST request to /verify-token.");
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
    pub jar: CookieJar,
    pub config: Config,
    pub server: axum_test::TestServer,
}

impl TestApp {
    pub async fn new(config: &Config) -> Self {
        let app = Application::build_router(config)
            .await
            .expect("Failed to build application.");
        let server = axum_test::TestServer::new(app).expect("Failed to start test server.");
        Self {
            jar: CookieJar::new(),
            config: config.clone(),
            server,
        }
    }

    pub async fn get_root(&self) -> TestResponse {
        self.server.get("/").await
    }

    pub async fn get_livez(&self) -> TestResponse {
        self.server.get("/livez").await
    }

    pub async fn get_healthz(&self) -> TestResponse {
        self.server.get("/healthz").await
    }

    pub fn post_login<Body>(&self, body: &Body) -> TestRequest
    where
        Body: serde::Serialize,
    {
        self.server.post("/login").json(body)
    }

    pub fn post_signup<Body>(&self, body: &Body) -> TestRequest
    where
        Body: serde::Serialize,
    {
        self.server.post("/signup").json(body)
    }

    pub fn post_logout(&self) -> TestRequest {
        self.server.post("/logout")
    }

    pub fn post_verify_2fa<Body>(&self, body: &Body) -> TestRequest
    where
        Body: serde::Serialize,
    {
        self.server.post("/verify-2fa").json(body)
    }

    pub fn post_verify_token<Body>(&self, body: &Body) -> TestRequest
    where
        Body: serde::Serialize,
    {
        self.server.post("/verify-token").json(body)
    }
}

use tokio::sync::OnceCell;

use crate::helpers::TestApp;
use libauth_service::config::Config;
use serde_json::Value;

static APP: OnceCell<TestApp> = OnceCell::const_new();

async fn get_test_app() -> &'static TestApp {
    APP.get_or_init(|| async { TestApp::new(&Config::default()).await })
        .await
}

/// Doesn't work because can't have 2 concurrent servers - needs static single instance
/// "Address already in use error"
/// even with static - need mutex? to make sure there's only one instance running
// static SERVER: OnceCell<TestServer> = OnceCell::const_new();
// async fn get_test_server() -> &'static TestServer {
//     SERVER
//         .get_or_init(|| async { TestServer::new(&Config::default()).await })
//         .await
// }
// #[tokio::test]
// async fn test_livez_returns_ok_server() {
//     let server = get_test_server().await;
//     let response = server.get_livez().await;
//     let status = response.status();
//     let headers = response.headers().clone();
//     let body = response
//         .json::<Value>()
//         .await
//         .expect("Response body is not valid JSON");
//     assert_eq!(status, reqwest::StatusCode::OK);
//     assert_eq!(headers.get("Content-Type").unwrap(), "application/json");
//     assert_eq!(body.get("status").unwrap(), "Alive");
// }
//
// #[tokio::test]
// async fn root_returns_auth_ui_server() {
//     let server = get_test_server().await;
//     let response = server.get_root().await;
//     assert_eq!(response.status(), reqwest::StatusCode::OK);
//     assert_eq!(
//         response.headers().get("Content-Type").unwrap(),
//         "text/html; charset=utf-8"
//     );
// }

#[tokio::test]
async fn root_returns_auth_ui_app() {
    let app = get_test_app().await;
    let response = app.get_root().await;
    assert_eq!(response.status_code(), reqwest::StatusCode::OK);
    assert_eq!(
        response.headers().get("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
}

#[tokio::test]
async fn test_livez_returns_ok_app() {
    let app = get_test_app().await;
    let response = app.get_livez().await;
    let body: Value =
        serde_json::from_str(&response.text()).expect("Response body is not valid JSON");
    assert_eq!(response.status_code(), reqwest::StatusCode::OK);
    assert_eq!(
        response.headers().get("Content-Type").unwrap(),
        "application/json"
    );
    assert_eq!(body.get("status").unwrap(), "Alive");
}

#[tokio::test]
async fn test_login() {
    let app = get_test_app().await;
    let response = app.post_login().await;
    assert_eq!(response.status_code(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn test_signup() {
    let app = get_test_app().await;
    let response = app.post_signup().await;
    assert_eq!(response.status_code(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn test_logout() {
    let app = get_test_app().await;
    let response = app.post_logout().await;
    assert_eq!(response.status_code(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn test_verify_2fa() {
    let app = get_test_app().await;
    let response = app.post_verify_2fa().await;
    assert_eq!(response.status_code(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn test_verify_token() {
    let app = get_test_app().await;
    let response = app.post_verify_token().await;
    assert_eq!(response.status_code(), reqwest::StatusCode::OK);
}

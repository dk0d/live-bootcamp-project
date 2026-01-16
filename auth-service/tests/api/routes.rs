use crate::helpers::{TestApp, TestServer};
use libauth_service::config::Config;
use serde_json::Value;

#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestServer::new(&Config::default()).await;
    let response = app.get_root().await;

    assert_eq!(response.status(), reqwest::StatusCode::OK);
    assert_eq!(
        response.headers().get("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
}

#[tokio::test]
async fn root_returns_auth_ui_app() {
    let app = TestApp::new(&Config::default()).await;
    let response = app.get_root().await;

    assert_eq!(response.status_code(), reqwest::StatusCode::OK);
    assert_eq!(
        response.headers().get("Content-Type").unwrap(),
        "text/html; charset=utf-8"
    );
}

/// Doesn't work because can't have 2 concurrent servers - needs static single instance
// #[tokio::test]
// async fn test_livez_returns_ok() {
//     let app = TestServer::new(&Config::default()).await;
//
//     let response = app.get_livez().await;
//     let status = response.status();
//     let headers = response.headers().clone();
//
//     let body = response
//         .json::<Value>()
//         .await
//         .expect("Response body is not valid JSON");
//
//     assert_eq!(status, reqwest::StatusCode::OK);
//     assert_eq!(headers.get("Content-Type").unwrap(), "application/json");
//     assert_eq!(body.get("status").unwrap(), "Alive");
// }

#[tokio::test]
async fn test_livez_returns_ok_app() {
    let app = TestApp::new(&Config::default()).await;
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
    let response = TestApp::new(&Config::default()).await.post_login().await;
    assert_eq!(response.status_code(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn test_signup() {
    let response = TestApp::new(&Config::default()).await.post_signup().await;
    assert_eq!(response.status_code(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn test_logout() {
    let response = TestApp::new(&Config::default()).await.post_logout().await;
    assert_eq!(response.status_code(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn test_verify_2fa() {
    let response = TestApp::new(&Config::default())
        .await
        .post_verify_2fa()
        .await;
    assert_eq!(response.status_code(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn test_verify_token() {
    let response = TestApp::new(&Config::default())
        .await
        .post_verify_token()
        .await;
    assert_eq!(response.status_code(), reqwest::StatusCode::OK);
}

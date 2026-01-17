use crate::helpers::get_test_app;
use serde_json::Value;

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

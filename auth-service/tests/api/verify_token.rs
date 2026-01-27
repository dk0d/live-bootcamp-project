use crate::helpers::get_test_app;

#[tokio::test]
async fn test_verify_token_200() {
    let app = get_test_app().await;
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJnaWxkYV9kZWJpdGlzQGhvdG1haWwuY29tIiwiZXhwIjoxNzY5NjA2Mzg0fQ.vTWN30CyMZcLydeWQV6NVUANehVgCozKNQwPczZOoUc";
    let body = serde_json::json!({ "token": token });
    let response = app.post_verify_token(&body).await;
    assert_eq!(response.status_code(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn test_verify_token_401_if_malformed_input() {
    let app = get_test_app().await;

    let test_cases = [
        serde_json::json!({"token": "'12345'"}),
        serde_json::json!({"token": "ereioadi"}),
        serde_json::json!({"token": "hello"}),
    ];

    for case in test_cases.iter() {
        let response = app.post_verify_token(case).await;
        assert_eq!(response.status_code(), reqwest::StatusCode::UNAUTHORIZED);
    }
}

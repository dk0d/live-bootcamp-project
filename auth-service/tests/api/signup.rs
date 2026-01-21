use crate::helpers::get_test_app;

#[tokio::test]
async fn test_signup_return_201_input_valid() {
    let app = get_test_app().await;
    let body = serde_json::json!({
        "method": "email_password",
        "email": "testuser@me.com",
        "password": "password123",
        "requires_2fa": false
    });
    let response = app.post_signup(&body).await;
    dbg!(&response);
    assert_eq!(response.status_code(), reqwest::StatusCode::CREATED);
}

#[tokio::test]
pub async fn test_signup_return_422_input_invalid() {
    let app = get_test_app().await;

    let test_cases = [
        serde_json::json!({}), // Empty body
        serde_json::json!({
            "password": "password123",
            "requires_2fa": false

        }),
        serde_json::json!({
            "username": "testuser",
            "password": "password123",
            "requires_2fa": false
        }),
        serde_json::json!({
            "method": "passkey",
            "username": "testuser",
            "password": "password123",
            "requires_2fa": false
        }),
    ];

    for body in test_cases.iter() {
        let response = app.post_signup(body).await;
        assert_eq!(
            response.status_code(),
            reqwest::StatusCode::UNPROCESSABLE_ENTITY
        );
    }
}

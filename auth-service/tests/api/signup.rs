use crate::helpers::get_test_app;

#[tokio::test]
async fn test_signup_return_201_input_valid() {
    let app = get_test_app().await;
    let body = serde_json::json!({
        "method": "email_password",
        "email": "testuser@me.com",
        "password": "password123",
        "two_factor": "none",
    });
    let response = app.post_signup(&body).await;
    dbg!(&response);
    assert_eq!(response.status_code(), reqwest::StatusCode::CREATED);
}

#[tokio::test]
pub async fn test_signup_return_422_input_invalid() {
    let app = get_test_app().await;

    let test_cases = [
        // Empty body
        serde_json::json!({}),
        // incomplete
        serde_json::json!({
            "password": "password123",
            "two_factor": "none"
        }),
        // missing method
        serde_json::json!({
            "username": "testuser@me.com",
            "password": "password123",
            "two_factor": "none"
        }),
        // bad key
        serde_json::json!({
            "method": "email_password",
            "username": "testuser",
            "password": "password123",
            "two_factor": "none"
        }),
        // invalid method/input pair
        serde_json::json!({
            "method": "passkey",
            "email": "testuser",
            "password": "password123",
            "two_factor": "none"
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

#[tokio::test]
pub async fn test_signup_return_400_invalid_input() {
    let app = get_test_app().await;
    let body = serde_json::json!({
        "method": "email_password",
        "email": "not-an-email",
        "password": "validpassword1234",
        "two_factor": "none"
    });
    let response = app.post_signup(&body).await;
    assert_eq!(response.status_code(), reqwest::StatusCode::BAD_REQUEST);
    let body = serde_json::json!({
        "method": "email_password",
        "email": "valid@email.com",
        "password": "badpwd",
        "two_factor": "none"
    });
    let response = app.post_signup(&body).await;
    assert_eq!(response.status_code(), reqwest::StatusCode::BAD_REQUEST);
}

#[tokio::test]
pub async fn test_signup_return_409_user_exists() {
    let app = get_test_app().await;

    let body = serde_json::json!({
        "method": "email_password",
        "email": "my@me.com",
        "password": "validpassword1234",
        "two_factor": "none"
    });

    let response = app.post_signup(&body).await;
    assert_eq!(response.status_code(), reqwest::StatusCode::CREATED);

    let response = app.post_signup(&body).await;
    assert_eq!(response.status_code(), reqwest::StatusCode::CONFLICT);
}

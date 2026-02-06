use lgr_auth::domain::Email;
use lgr_auth::utils::auth::generate_auth_token;

use crate::helpers::get_test_app;

#[tokio::test]
async fn test_verify_token_200() {
    let app = get_test_app().await;
    let email = Email::parse("tester@test.com").expect("valid email");
    let token = generate_auth_token(&email, &app.config.jwt.secret).expect("valid token");
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

#[tokio::test]
async fn test_verify_token_401_if_banned_token() {
    let app = get_test_app().await;

    // Signup
    let response = app
        .post_signup(&serde_json::json!({
            "method": "email_password",
            "email": "user@test.com",
            "password": "StrongP@ssw0rd!",
            "two_factor": "none",
        }))
        .await;

    assert_eq!(response.status_code(), reqwest::StatusCode::CREATED);

    // Login
    let response = app
        .post_login(&serde_json::json!({
            "method": "email_password",
            "email": "user@test.com",
            "password": "StrongP@ssw0rd!",
        }))
        // Makes sure the token cookie is around for logout request
        .save_cookies()
        .await;
    assert_eq!(response.status_code(), reqwest::StatusCode::OK);

    // Grab token for future verification test
    let token = response.cookie(&app.config.jwt.cookie_name);
    let token = token.value();

    // Logout
    let response = app.post_logout().await;
    assert_eq!(response.status_code(), reqwest::StatusCode::OK);

    // Ensure token is now banned
    let body = serde_json::json!({ "token": token });
    let response = app.post_verify_token(&body).await;
    assert_eq!(response.status_code(), reqwest::StatusCode::UNAUTHORIZED);
}

use fake::{faker, Fake};
use reqwest::StatusCode;

use crate::helpers::get_test_app;

#[tokio::test]
async fn test_login_422_if_malformed_credentials() {
    let app = get_test_app().await;

    let test_cases = [
        // Empty body
        serde_json::json!({}),
        // Incomplete
        serde_json::json!({
            "password": "password123",
        }),
        // Invalid method/Input pair
        serde_json::json!({
            "method": "magic",
            "password": "password123",
        }),
    ];

    for body in test_cases.iter() {
        let response = app.post_login(&body).await;
        assert_eq!(
            response.status_code(),
            reqwest::StatusCode::UNPROCESSABLE_ENTITY
        );
    }
}

#[tokio::test]
async fn test_login_400_if_malformed_credentials() {
    let app = get_test_app().await;

    let test_cases = [
        // Invalid email
        serde_json::json!({
            "method": "email_password",
            "email": "123897@.1",
            "password": "password123",
        }),
        // Invalid email
        serde_json::json!({
            "method": "email_password",
            "email": "@.com",
            "password": "password123",
        }),
        // Too short
        serde_json::json!({
            "method": "email_password",
            "email": "you@me.com",
            "password": "123",
        }),
    ];

    for body in test_cases.iter() {
        let response = app.post_login(&body).await;
        dbg!(&response);
        assert_eq!(response.status_code(), reqwest::StatusCode::BAD_REQUEST);
    }
}

#[tokio::test]
async fn test_login_401_if_invalid_credentials() {
    let app = get_test_app().await;

    app.post_signup(&serde_json::json!({
        "method": "email_password",
        "email": "can@login.com",
        "password": "password123"
    }))
    .await;

    let test_cases = [
        serde_json::json!({
            "method": "email_password",
            "email": "you@me.com",
            "password": "password123",
        }),
        serde_json::json!({
            "method": "email_password",
            "email": "can@login.com",
            "password": "badpassword123",
        }),
    ];

    for body in test_cases.iter() {
        let response = app.post_login(&body).await;
        dbg!(&response);
        assert_eq!(response.status_code(), reqwest::StatusCode::UNAUTHORIZED);
    }

    // Non-existent user
}

fn get_random_email() -> String {
    faker::internet::en::FreeEmail().fake()
}

#[tokio::test]
async fn test_login_200_if_valid_credentials_and_2fa_disabled() {
    let app = get_test_app().await;
    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "method": "email_password",
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status_code(), StatusCode::CREATED);
    let login_body = serde_json::json!({
        "method": "email_password",
        "email": random_email,
        "password": "password123",
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    let cookies = response.cookies();
    let auth_cookie = cookies
        .get(&app.config.jwt.cookie_name)
        .expect("No auth cookie found");
    assert!(!auth_cookie.value().is_empty());
}

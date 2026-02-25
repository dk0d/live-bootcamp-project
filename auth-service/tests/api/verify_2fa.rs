use base64::Engine;
use base64::engine::GeneralPurpose;
use lgr_auth::domain::{EmailTemplate, TwoFactorEmailData};
use lgr_auth::routes::LoginResponse;
use lgr_auth::utils::auth::TwoFAClaims;
use reqwest::Url;

use crate::common::get_test_app;

#[tokio::test]
async fn test_verify_2fa_206() {
    let app = get_test_app().await;
    let body = serde_json::json!({
        "method": "email_password",
        "email": "testuser206@me.com",
        "password": "password123",
        "two_factor": "email",
    });
    let response = app.post_signup(&body).await;
    dbg!(&response);
    assert_eq!(response.status_code(), reqwest::StatusCode::CREATED);

    let body = serde_json::json!({
        "method": "email_password",
        "email": "testuser206@me.com",
        "password": "password123",
    });

    let response = app.post_login(&body).await;
    let response_body: LoginResponse = response.json::<LoginResponse>();
    assert_eq!(response.status_code(), reqwest::StatusCode::PARTIAL_CONTENT);
    assert!(matches!(response_body, LoginResponse::TwoFactor { .. }));
}

fn parse_email_data(data: &TwoFactorEmailData) -> (String, String) {
    let url = Url::parse(&data.redirect_url).expect("valid url");
    let payload = url
        .query_pairs()
        .find_map(|(key, value)| {
            if key == "payload" {
                return Some(value);
            }
            None
        })
        .expect("jwt payload");
    let engine = GeneralPurpose::new(
        &base64::alphabet::URL_SAFE,
        base64::engine::general_purpose::PAD,
    );
    let payload = engine.decode(payload.as_ref()).expect("decoded");
    let payload = str::from_utf8(&payload).expect("string");
    let jwt = &jsonwebtoken::dangerous::insecure_decode::<TwoFAClaims>(payload)
        .expect("valid token data");
    (data.code.clone(), jwt.claims.sub.as_ref().to_string())
}

#[tokio::test]
async fn test_verify_2fa_200() {
    let app = get_test_app().await;
    let body = serde_json::json!({
        "method": "email_password",
        "email": "testuser200@me.com",
        "password": "password123",
        "two_factor": "email",
    });
    let response = app.post_signup(&body).await;
    assert_eq!(response.status_code(), reqwest::StatusCode::CREATED);

    let body = serde_json::json!({
        "method": "email_password",
        "email": "testuser200@me.com",
        "password": "password123",
    });

    let response = app.post_login(&body).await;
    assert_eq!(response.status_code(), reqwest::StatusCode::PARTIAL_CONTENT);
    let response_body: LoginResponse = response.json::<LoginResponse>();
    assert!(matches!(response_body, LoginResponse::TwoFactor { .. }));

    let email = app
        .emails
        .0
        .lock()
        .expect("email sent")
        .last()
        .cloned()
        .expect("has email");

    let (code, id) = match email.template {
        EmailTemplate::TwoFactor(data) => parse_email_data(&data),
    };
    let body = serde_json::json!({
        "method": "email",
        "email": "testuser200@me.com",
        "id": id,
        "code": code,
    });
    let response = app.post_verify_2fa(&body).await;
    assert_eq!(response.status_code(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn test_verify_2fa_401() {
    let app = get_test_app().await;
    let body = serde_json::json!({
        "method": "email_password",
        "email": "testuser401@me.com",
        "password": "password123",
        "two_factor": "email",
    });
    let response = app.post_signup(&body).await;
    assert_eq!(response.status_code(), reqwest::StatusCode::CREATED);

    let body = serde_json::json!({
        "method": "email_password",
        "email": "testuser401@me.com",
        "password": "password123",
    });

    let response = app.post_login(&body).await;
    assert_eq!(response.status_code(), reqwest::StatusCode::PARTIAL_CONTENT);
    let response_body: LoginResponse = response.json::<LoginResponse>();
    assert!(matches!(response_body, LoginResponse::TwoFactor { .. }));

    let email = app
        .emails
        .0
        .lock()
        .expect("email sent")
        .last()
        .cloned()
        .expect("has email");

    let (code, id) = match email.template {
        EmailTemplate::TwoFactor(data) => parse_email_data(&data),
    };

    if let LoginResponse::TwoFactor { .. } = response_body {
        let failure_cases = [
            serde_json::json!({
                "method": "email",
                "email": "testuser401@me.com",
                "id": id,
                "code": "000000",
            }),
            serde_json::json!({
                "method": "email",
                "email": "wrong_email@me.com",
                "id": id,
                "code": code,
            }),
            serde_json::json!({
                "method": "email",
                "email": "testuser401@me.com",
                "id": "bad-id",
                "code": code,
            }),
            serde_json::json!({
                "method": "email",
                "email": "testuser401@me.com",
                "id": "dc5b25ca-1d7b-4827-8843-c2d1ab9d0f7f",
                "code": code,
            }),
        ];

        for case in failure_cases.iter() {
            let response = app.post_verify_2fa(case).await;
            dbg!(&response);
            assert_eq!(response.status_code(), reqwest::StatusCode::UNAUTHORIZED);
        }
    }
}

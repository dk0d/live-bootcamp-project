use lgr_auth::routes::LoginResponse;

use crate::helpers::get_test_app;

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

    // FIXME: This test is currently broken because the 2FA code is generated and sent to the email,
    // which we can't access in the test. To properly test this, we would need to mock the email
    // sending functionality to capture the generated 2FA code and use it in the verification step.
    //
    // if let LoginResponse::TwoFactor { method, url, .. } = response_body {
    //     let body = serde_json::json!({
    //         "method": "email",
    //         "email": "testuser200@me.com",
    //         "id": id,
    //         "code": code,
    //     });
    //     let response = app.post_verify_2fa(&body).await;
    //     assert_eq!(response.status_code(), reqwest::StatusCode::OK);
    // }
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

    // FIXME: This test is currently broken because the 2FA code is generated and sent to the email,
    // if let LoginResponse::TwoFactor { url, .. } = response_body {
    //     let failure_cases = [
    //         serde_json::json!({
    //             "method": "email",
    //             "email": "testuser401@me.com",
    //             "id": id,
    //             "code": "000000",
    //         }),
    //         serde_json::json!({
    //             "method": "email",
    //             "email": "wrong_email@me.com",
    //             "id": id,
    //             "code": code,
    //         }),
    //         serde_json::json!({
    //             "method": "email",
    //             "email": "testuser401@me.com",
    //             "id": "bad-id",
    //             "code": code,
    //         }),
    //         serde_json::json!({
    //             "method": "email",
    //             "email": "testuser401@me.com",
    //             "id": "dc5b25ca-1d7b-4827-8843-c2d1ab9d0f7f",
    //             "code": code,
    //         }),
    //     ];
    //
    //     for case in failure_cases.iter() {
    //         let response = app.post_verify_2fa(case).await;
    //         dbg!(&response);
    //         assert_eq!(response.status_code(), reqwest::StatusCode::UNAUTHORIZED);
    //     }
    // }
}

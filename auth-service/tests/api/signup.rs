use crate::helpers::get_test_app;

// TODO: Re-enable this test once the signup functionality is implemented.
// #[tokio::test]
// async fn test_signup_200() {
//     let app = get_test_app().await;
//     let body = serde_json::json!({
//         "username": "testuser",
//         "password": "password123",
//         "requires_2fa": false
//     });
//     let response = app.post_signup(&body).await;
//     assert_eq!(response.status_code(), reqwest::StatusCode::OK);
// }

#[tokio::test]
pub async fn test_signup_return_422_if_malformed_input() {
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

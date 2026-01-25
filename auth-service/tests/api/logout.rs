use crate::helpers::get_test_app;
use cookie::{CookieBuilder, CookieJar};
use lgr_auth::domain::Email;
use lgr_auth::utils::auth::generate_auth_cookie;

#[tokio::test]
async fn test_logout_200_if_valid_jwt() {
    let app = get_test_app().await;
    let email = Email::parse("can@logout.com").unwrap();
    let mock_token =
        generate_auth_cookie(&email, &app.config.jwt).expect("Failed to generate auth cookie");
    let mut server_jar = CookieJar::default();
    server_jar.add(mock_token);
    let response = app.post_logout().add_cookies(server_jar).await;
    assert_eq!(response.status_code(), reqwest::StatusCode::OK);
    // Verify that the JWT cookie is cleared
    assert_eq!(
        response
            .cookies()
            .get(&app.config.jwt.cookie_name)
            .map(|c| c.value().to_string()),
        Some("".to_string())
    );
}

#[tokio::test]
async fn test_logout_400_if_jwt_cookie_missing() {
    let app = get_test_app().await;
    let response = app.post_logout().await;
    assert_eq!(response.status_code(), reqwest::StatusCode::BAD_REQUEST);
}

// Not sure what this test is for?
#[tokio::test]
async fn test_logout_400_if_called_twice_in_a_row() {
    let app = get_test_app().await;
    let email = Email::parse("can@logout.com").unwrap();
    let mock_token =
        generate_auth_cookie(&email, &app.config.jwt).expect("Failed to generate auth cookie");
    let mut server_jar = CookieJar::default();
    server_jar.add(mock_token);
    let response = app.post_logout().add_cookies(server_jar).await;
    assert_eq!(response.status_code(), reqwest::StatusCode::OK);
    // Verify that the JWT cookie is cleared
    assert_eq!(
        response
            .cookies()
            .get(&app.config.jwt.cookie_name)
            .map(|c| c.value().to_string()),
        Some("".to_string())
    );
    let response = app.post_logout().await;
    assert_eq!(response.status_code(), reqwest::StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_logout_401_if_invalid_token() {
    let app = get_test_app().await;
    let mut jar = CookieJar::default();
    let cookie = CookieBuilder::new(
        app.config.jwt.cookie_name.clone(),
        "invalid_token".to_string(),
    )
    .http_only(true)
    .same_site(cookie::SameSite::Lax)
    .secure(true)
    .path("/")
    .build();
    jar.add(cookie);
    let response = app.post_logout().add_cookies(jar).await;
    assert_eq!(response.status_code(), reqwest::StatusCode::UNAUTHORIZED);
}

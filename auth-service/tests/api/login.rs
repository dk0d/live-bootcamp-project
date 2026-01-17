use crate::helpers::get_test_app;

#[tokio::test]
async fn test_login() {
    let app = get_test_app().await;
    let response = app.post_login().await;
    assert_eq!(response.status_code(), reqwest::StatusCode::OK);
}

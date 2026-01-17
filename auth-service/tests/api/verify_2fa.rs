use crate::helpers::get_test_app;


#[tokio::test]
async fn test_verify_2fa() {
    let app = get_test_app().await;
    let response = app.post_verify_2fa().await;
    assert_eq!(response.status_code(), reqwest::StatusCode::OK);
}

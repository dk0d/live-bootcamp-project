// Doesn't work because can't have 2 concurrent servers - needs static single instance
// "Address already in use error"
// even with static - need mutex? to make sure there's only one instance running
// static SERVER: OnceCell<TestServer> = OnceCell::const_new();
// async fn get_test_server() -> &'static TestServer {
//     SERVER
//         .get_or_init(|| async { TestServer::new(&Config::default()).await })
//         .await
// }
// #[tokio::test]
// async fn test_livez_returns_ok_server() {
//     let server = get_test_server().await;
//     let response = server.get_livez().await;
//     let status = response.status();
//     let headers = response.headers().clone();
//     let body = response
//         .json::<Value>()
//         .await
//         .expect("Response body is not valid JSON");
//     assert_eq!(status, reqwest::StatusCode::OK);
//     assert_eq!(headers.get("Content-Type").unwrap(), "application/json");
//     assert_eq!(body.get("status").unwrap(), "Alive");
// }
//
// #[tokio::test]
// async fn root_returns_auth_ui_server() {
//     let server = get_test_server().await;
//     let response = server.get_root().await;
//     assert_eq!(response.status(), reqwest::StatusCode::OK);
//     assert_eq!(
//         response.headers().get("Content-Type").unwrap(),
//         "text/html; charset=utf-8"
//     );
// }

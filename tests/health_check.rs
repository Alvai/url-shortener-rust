mod test_utils;

use test_utils::spawn_app::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let adrr = spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get([&adrr, "/health_check"].concat())
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

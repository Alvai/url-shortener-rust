mod test_utils;

use reqwest::Client;
use test_utils::spawn_app::spawn_app;

#[tokio::test]
async fn greet_endpoints_works() {
    spawn_app();

    let client = reqwest::Client::new();

    test_greet_endpoint(&client, "/", "Hello World!").await;
    test_greet_endpoint(&client, "/Alvi", "Hello Alvi!").await;
}

async fn test_greet_endpoint(client: &Client, path: &str, response_text: &str) {
    let response = client
        .get(["http://127.0.0.1:8000", path].concat())
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());

    let content = response.text().await.expect("Failed to read reponse text");
    assert_eq!(response_text, content);
}

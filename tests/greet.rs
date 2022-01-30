mod test_utils;

use reqwest::Client;
use test_utils::spawn_app::spawn_app;

#[tokio::test]
async fn greet_endpoints_works() {
    let addr = spawn_app();

    let client = reqwest::Client::new();

    test_greet_endpoint(&client, [&addr, "/"].concat(), "Hello World!").await;
    test_greet_endpoint(&client, [&addr, "/Alvi"].concat(), "Hello Alvi!").await;
}

async fn test_greet_endpoint(client: &Client, path: String, response_text: &str) {
    let response = client
        .get(path)
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());

    let content = response.text().await.expect("Failed to read reponse text");
    assert_eq!(response_text, content);
}

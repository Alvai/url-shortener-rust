use std::collections::HashMap;

use rstest::rstest;

mod test_utils;

use test_utils::spawn_app::spawn_app;

#[tokio::test]
async fn encode_link_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let mut body = HashMap::new();
    body.insert("name", "alvi");
    body.insert("url", "https://alvi.dev");

    // Act
    let response = client
        .post(&format!("{}/encode_link", &app_address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[rstest]
#[case({
    let mut body = HashMap::new();
    body.insert("url", "https://alvi.dev");
    body
}, "missing the name")]
#[case({
    let mut body = HashMap::new();
    body.insert("name", "alvi");
    body
}, "missing the url")]
#[case({
    let body = HashMap::new();
    body
}, "missing both the url and name")]
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing(
    #[case] body: HashMap<&str, &str>,
    #[case] error_message: String,
) {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/encode_link", &app_address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(
        400,
        response.status().as_u16(),
        // Additional customised error message on test failure
        "The API did not fail with 400 Bad Request when the payload was {}.",
        error_message
    )
}

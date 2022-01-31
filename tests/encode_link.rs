use rstest::rstest;

mod test_utils;

use test_utils::spawn_app::spawn_app;

#[tokio::test]
async fn encode_link_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    // Act
    let body = "url=https%3A%2F%2Falvi.dev%2Ffr&name=Alvi";
    let response = client
        .post(&format!("{}/encode_link", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[rstest]
#[case("url=https%3A%2F%2Falvi.dev%2Ffr", "missing the name")]
#[case("name=Alvi", "missing the url")]
#[case("", "missing both the url and name")]
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing(
    #[case] error_message: String,
    #[case] t: String,
) {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/encode_link", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(t.clone())
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

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    // We need to bring in`reqwest`
    // to perform HTTP requests against our application.
    //
    // Use`cargo add reqwest --dev --vers 0.11` to add
    // it under`[dev-dependencies]` in Cargo.toml
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
// Launch our application in the background ~somehow~
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address"); // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    //
    // New dev dependency - let's add tokio to the party with
    //`cargo add tokio --dev --vers 1`
    let _ = tokio::spawn(server);
}

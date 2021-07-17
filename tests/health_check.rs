//! tests/
//!

#[actix_rt::test]
async fn health_check_works() {
    // Arange
    spawn_app();
    let client = reqwest::Client::new();

    // Act
    let target_uri = "http://127.0.0.1:8000/health_check";
    let response = client
        .get(target_uri)
        .send()
        .await
        .unwrap_or_else(|_| panic!("Failed to complete request to {}", target_uri));

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zerotoprod::run().expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
}

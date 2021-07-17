//! tests/
//!

use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
    // Arange
    let address = spawn_app();
    let client = reqwest::Client::new();
    let target_uri = format!("{}/health_check", &address);

    // Act
    let response = client
        .get(&target_uri)
        .send()
        .await
        .unwrap_or_else(|_| panic!("Failed to complete request to {}", target_uri));

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    // retrieve port assigned by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zerotoprod::run(listener).expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
    // and return the application server address to the caller
    format!("http://127.0.0.1:{}", port)
}

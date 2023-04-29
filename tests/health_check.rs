//! tests/health_check.rs

#[tokio::test]
async fn health_check_test() {
    // Arrange
    spawn_app();

    // Using reqwest to make HTTP requests to our application.
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:8000/health_check")
        .send()
        .await
        .expect("failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero2prod::run("127.0.0.1:0").expect("failed to bind address");
    tokio::spawn(server);
}

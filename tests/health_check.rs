//! tests/health_check.rs

#[tokio::test]
async fn health_check_test() {
    // Arrange

    let url = spawn_app();
    // Using reqwest to make HTTP requests to our application.
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &url))
        .send()
        .await
        .expect("failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    let port = listener.local_addr().expect("failed to get port").port();
    let server = zero2prod::run(listener).expect("failed to bind address");
    tokio::spawn(server);
    format!("http://localhost:{}", port)
}

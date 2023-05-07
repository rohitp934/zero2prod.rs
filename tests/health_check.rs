//! tests/health_check.response

use sqlx::{Connection, PgConnection};
use zero2prod::{configuration::get_config, startup::run};

fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    let port = listener.local_addr().expect("failed to get port").port();
    let server = run(listener).expect("failed to bind address");
    tokio::spawn(server);
    format!("http://localhost:{}", port)
}

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

#[tokio::test]
async fn subscribe_returns_200_for_valid_form() {
    let url = spawn_app();
    let config = get_config().expect("Unable to read from config file");
    let connection_string = config.database.connection_string();

    let mut conn = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to postgres db.");
    let client = reqwest::Client::new();

    let request_body = "name=scooby%20doo&email=lingardium_espinoza%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", &url))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(request_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut conn)
        .await
        .expect("Failed to fetch from the db");

    assert_eq!(saved.email, "lingardium_espinoza@gmail.com");
    assert_eq!(saved.name, "scooby doo");
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid_form() {
    let url = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = [
        ("name=scooby%20doo", "missing the email"),
        ("email=lingardium_espinoza%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (request_body, error_msg) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", &url))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(request_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Verify that response status is 400.
        assert_eq!(
            400,
            response.status().as_u16(),
            "The api did not fail with a 400 Bad Request when the payload was {}",
            error_msg
        );
    }
}

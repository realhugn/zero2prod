use std::net::TcpListener;

use sqlx::{Connection, PgConnection};
use zero2prod::configuration::get_config;

#[tokio::test]
async fn health_check_works() {
    let add = spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &add))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    //Arrange
    let app_address = spawn_app();
    let config = get_config().expect("Failed to read configuration");
    let connection_string = config.database.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");
    let client = reqwest::Client::new();

    // Begin Test
    let body = "name=real%20hung&email=realhugn%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(200, response.status().as_u16());

    let inserted = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch inserted.");

    assert_eq!(inserted.email, "realhugn@email.com");
    assert_eq!(inserted.name, "real hung")
}

fn spawn_app() -> String {
    let listenser = TcpListener::bind("127.0.0.1:0").expect("Failed to bind port");

    let port = listenser.local_addr().unwrap().port();
    let server = zero2prod::run(listenser).expect("Failed to bind address");

    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
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
        );
    }
}

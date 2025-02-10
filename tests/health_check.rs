use subscriptions::run;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_success() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let body = "email=admin@example.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscribe_fail() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("email=admin", "invalid email form"),
        ("", "missing the email")
    ];
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "Error: {}.",
            error_message
        );
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let server = run(listener).unwrap();
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

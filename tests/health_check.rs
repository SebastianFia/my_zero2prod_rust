use reqwest;
use std::net::TcpListener;

fn spawn_app_and_get_address() -> String {
    //bind at port 0 to get random port from os
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener
        .local_addr()
        .expect("failed to get socket address")
        .port();
    let server = zero2prod::build_server(listener).expect("failed to build server");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let app_address = spawn_app_and_get_address();
    let client = reqwest::Client::new();
    let health_check_url = format!("{}/health_check", app_address);

    let response = client
        .get(&health_check_url)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app_and_get_address();
    let client = reqwest::Client::new();
    let subscribtions_url = format!("{}/subscriptions", app_address);

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(subscribtions_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app_and_get_address();
    let client = reqwest::Client::new();
    let subscriptions_url = format!("{}/subscriptions", app_address);

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, cause_of_400) in test_cases.into_iter() {
        let response = client
            .post(&subscriptions_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            cause_of_400
        );
    }
}

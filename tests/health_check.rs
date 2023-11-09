use reqwest;
use zero2prod::{HTTP_PORT, LOCAL_HOST};

#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();
    let health_check_url = format!("{}:{}/health_check", LOCAL_HOST, HTTP_PORT);

    let response = client
        .get(health_check_url)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero2prod::build_server().expect("failed to build server");
    let _ = tokio::spawn(server);
}

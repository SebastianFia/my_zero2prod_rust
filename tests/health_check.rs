use reqwest;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let server_address = spawn_app_and_get_address();
    let client = reqwest::Client::new();
    let health_check_url = format!("{}/health_check", server_address);

    let response = client
        .get(&health_check_url)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app_and_get_address() -> String {
    //bind at port 0 to get random port from os
    let listener =  TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().expect("failed to get socket address").port();
    let server = zero2prod::build_server(listener).expect("failed to build server");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
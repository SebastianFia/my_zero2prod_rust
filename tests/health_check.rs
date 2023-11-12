use reqwest;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::DatabaseSettings;

pub struct TestApp {
    pub address: String,
    pub db_connection_pool: PgPool,
}

async fn spawn_test_app() -> TestApp {
    use zero2prod::build_server;
    use zero2prod::configuration::get_configuration;

    //create listener by binding at port 0 (to get random port from os)
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener
        .local_addr()
        .expect("Failed to get socket address")
        .port();
    let app_address = format!("http://127.0.0.1:{}", port);

    //create create postgres db and connection pool to it
    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let db_connection_pool = configure_db_and_get_connection_pool(&configuration.database).await;

    //build server and run it in tokio task
    let server =
        build_server(listener, db_connection_pool.clone()).expect("Failed to build server");
    let _ = tokio::spawn(server);

    TestApp {
        address: app_address,
        db_connection_pool,
    }
}

async fn configure_db_and_get_connection_pool(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database");

    connection_pool
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_test_app().await;
    let client = reqwest::Client::new();
    let health_check_url = format!("{}/health_check", &app.address);

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
    let app = spawn_test_app().await;
    let client = reqwest::Client::new();
    let subscribtions_url = format!("{}/subscriptions", &app.address);

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(subscribtions_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_connection_pool)
        .await
        .expect("Failed to fetch saved subscription from db");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_test_app().await;
    let client = reqwest::Client::new();
    let subscriptions_url = format!("{}/subscriptions", &app.address);

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%41gmail.com", "missing the name"),
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

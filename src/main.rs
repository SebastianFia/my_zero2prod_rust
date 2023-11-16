use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::{build_server, LOCAL_HOST};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("failed to read configuration");
    let server_address = format!("{}:{}", LOCAL_HOST, configuration.application_port);
    let listener = TcpListener::bind(server_address)?;
    let db_connection_pool =
        PgPool::connect(&configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres");

    build_server(listener, db_connection_pool)?.await
}

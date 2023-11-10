use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::{build_server, LOCAL_HOST};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("failed to read configuration");
    let server_address = format!("{}:{}", LOCAL_HOST, configuration.application_port);
    let listener = TcpListener::bind(server_address)?;

    build_server(listener)?.await
}

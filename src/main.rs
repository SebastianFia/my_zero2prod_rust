use std::net::TcpListener;

use zero2prod::{build_server, LOCAL_HOST, DEFAULT_PORT};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server_address = format!("{}:{}", LOCAL_HOST, DEFAULT_PORT);
    let listener = TcpListener::bind(server_address)?;

    build_server(listener)?.await
}

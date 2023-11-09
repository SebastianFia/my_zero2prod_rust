use zero2prod::build_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    build_server()?.await
}

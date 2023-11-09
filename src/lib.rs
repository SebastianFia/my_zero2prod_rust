pub const DEFAULT_PORT: &str = "8000";
pub const LOCAL_HOST: &str = "http://127.0.0.1";

pub mod build_server;
pub use build_server::build_server;

mod routes;

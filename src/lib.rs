pub const DEFAULT_PORT: &str = "8000";
pub const LOCAL_HOST: &str = "127.0.0.1";

pub mod build_server;
pub mod configuration;
pub mod routes;
pub mod log;

pub use build_server::build_server;
pub use log::init_logger;

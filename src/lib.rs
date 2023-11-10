pub const DEFAULT_PORT: &str = "8000";
pub const LOCAL_HOST: &str = "127.0.0.1";

pub mod routes;
pub mod startup;

pub use startup::build_server;

pub const DEFAULT_PORT: &str = "8000";
pub const LOCAL_HOST: &str = "127.0.0.1";

pub mod startup;
pub mod routes;

pub use startup::build_server;


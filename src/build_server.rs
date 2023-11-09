use crate::routes::{greet, health_check, subscribe};
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

use actix_web::dev::Server;

pub fn build_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/{name}", web::get().to(greet))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

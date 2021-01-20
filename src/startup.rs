use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{HttpServer, web, App};
use crate::routes;

// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/healthz", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
    }).listen(listener)?.run();
    Ok(server)
}

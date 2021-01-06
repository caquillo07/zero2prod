use actix_web::{web, App, Responder, HttpServer, HttpResponse};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub async fn run(port: &str) -> std::io::Result<()> {
    println!("running on port {}", port);
    HttpServer::new(|| {
        App::new()
            .route("/healthz", web::get().to(health_check))
    }).bind(port)?.
        run().
        await
}

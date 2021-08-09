use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration, startup};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // panic if we cant read the config
    let config = configuration::get_configuration()
        .expect("failed to read the configuration");

    let address = &format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to postgres");

    startup::run(listener, connection_pool)?.await
}

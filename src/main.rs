use std::net::TcpListener;
use zero2prod::{startup, configuration};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // panic if we cant read the config
    let config = configuration::get_configuration()
        .expect("failed to read the configuration");
    let address = &format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    let port = listener.local_addr().unwrap().port();
    println!("running on port {}", port);
    startup::run(listener)?.await
}

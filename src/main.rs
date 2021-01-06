use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").
        expect("failed to bind server address");
    let port = listener.local_addr().unwrap().port();
    println!("running on port {}", port);
    zero2prod::run(listener)?.await
}

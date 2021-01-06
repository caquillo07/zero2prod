#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    zero2prod::run("127.0.0.1:8080").await
}

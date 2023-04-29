//! src/main.rs
use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    println!(
        "Server listening on http://localhost:{}",
        listener.local_addr().unwrap().port()
    );
    run(listener)?.await
}

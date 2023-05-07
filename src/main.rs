//! src/main.rs

use zero2prod::{startup::run, configuration::get_config};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Unable to read from config file.");
    println!("{:?}", config);
    let address = format!("127.0.0.1:{}", config.port);
    let listener = std::net::TcpListener::bind(&address).expect("failed to bind random port");
    println!(
        "Server listening on http://localhost:{}",
        config.port
    );
    run(listener)?.await
}

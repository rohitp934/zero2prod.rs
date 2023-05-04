use actix_web::{dev::Server, App, HttpServer};

use crate::routes::config;

pub fn run(listener: std::net::TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().configure(config))
        .listen(listener)?
        .run();
    Ok(server)
}

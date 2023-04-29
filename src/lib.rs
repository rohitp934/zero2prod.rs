use actix_web::{dev::Server, get, App, HttpResponse, HttpServer, Responder};

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(addr: &str) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check))
        .bind(addr)?
        .run();
    Ok(server)
}

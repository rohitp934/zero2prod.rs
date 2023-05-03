use actix_web::{dev::Server, get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[derive(serde::Deserialize)]
struct Subscription {
    email: String,
    name: String,
}

#[post("/subscriptions")]
async fn subscribe(form: web::Form<Subscription>) -> impl Responder {
    println!("Subscription received for {} : {}", form.email, form.name);
    HttpResponse::Ok()
}

pub fn run(listener: std::net::TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscribe))
        .listen(listener)?
        .run();
    Ok(server)
}

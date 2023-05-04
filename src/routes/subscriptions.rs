use actix_web::{post, web, HttpResponse, Responder};

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

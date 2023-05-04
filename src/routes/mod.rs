//! src/routes/mod.rs

mod health_check;
mod subscriptions;

pub use health_check::*;
pub use subscriptions::*;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(health_check);
    cfg.service(subscribe);
}

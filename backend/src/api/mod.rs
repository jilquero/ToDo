pub mod task;
pub mod user;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(user::configure)
            .configure(task::configure),
    );
}

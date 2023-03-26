mod api;
mod domain;
mod infrastructure;

use crate::domain::tasks;
use crate::domain::users;
use crate::infrastructure::config::Config;
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::config::PersistentSession;
use actix_session::storage::RedisActorSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use time::Duration;

#[derive(Clone)]
pub struct ServerContext {
    pub users_service: Arc<users::service::Service>,
    pub tasks_service: Arc<tasks::service::Service>,
}

pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let config = Config::from_env().expect("Server configuration");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    let users_service = Arc::new(users::service::Service::new(pool.clone()));
    let tasks_service = Arc::new(tasks::service::Service::new(pool.clone()));
    let server_context = Arc::new(ServerContext {
        users_service,
        tasks_service,
    });

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .app_data(web::Data::new(Arc::clone(&server_context)))
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new(&config.redis_url),
                    Key::from(config.secret_key.as_bytes()),
                )
                .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(1)))
                .cookie_name("auth-twoja".to_owned())
                .cookie_secure(true)
                .cookie_domain(Some("localhost".to_string()))
                .cookie_path("/".to_owned())
                .build(),
            )
            .wrap(cors)
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .configure(api::configure)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}

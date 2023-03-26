use std::sync::Arc;

use actix_identity::Identity;
use actix_web::{get, patch, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use common::model::user::{CreateUser, LoginUser, UpdateUser, User as UserResponse};
use uuid::Uuid;
use validator::Validate;

use crate::{
    domain::{error::Error as ServiceError, users::user::User},
    ServerContext,
};

#[post("/register")]
async fn register(
    create_user: web::Json<CreateUser>,
    request: HttpRequest,
    ctx: web::Data<Arc<ServerContext>>,
) -> impl Responder {
    let create_user = create_user.into_inner();

    match create_user.validate() {
        Ok(_) => (),
        Err(e) => return HttpResponse::BadRequest().json(e),
    }

    let user = ctx.users_service.save(create_user).await;

    match user {
        Ok(user) => match Identity::login(&request.extensions(), user.user_uuid.to_string()) {
            Ok(_) => HttpResponse::Ok().json(UserResponse::from(user)),
            Err(_) => HttpResponse::InternalServerError().json("Unexpected error".to_string()),
        },
        Err(ServiceError::NotUnique(e)) => HttpResponse::Conflict().json(e),
        Err(ServiceError::Other(e)) => HttpResponse::InternalServerError().json(e),
        Err(_) => HttpResponse::InternalServerError().json("Unexpected error".to_string()),
    }
}

#[post("/login")]
async fn login(
    login_user: web::Json<LoginUser>,
    request: HttpRequest,
    ctx: web::Data<Arc<ServerContext>>,
) -> impl Responder {
    let login_user = login_user.into_inner();

    match login_user.validate() {
        Ok(_) => (),
        Err(e) => return HttpResponse::BadRequest().json(e),
    }

    let user = ctx.users_service.auth_user(login_user).await;

    match user {
        Ok(user) => match Identity::login(&request.extensions(), user.user_uuid.to_string()) {
            Ok(_) => HttpResponse::Ok().json(UserResponse::from(user)),
            Err(_) => HttpResponse::InternalServerError().json("Error".to_string()),
        },
        Err(ServiceError::Unauthorized) | Err(ServiceError::NotFound) => {
            HttpResponse::Unauthorized().json("Invalid password or email".to_string())
        }
        Err(ServiceError::Other(e)) => HttpResponse::InternalServerError().json(e),
        Err(_) => HttpResponse::InternalServerError().json("Unexpected error".to_string()),
    }
}

#[post("/logout")]
async fn logout(id: Identity) -> impl Responder {
    id.logout();
    HttpResponse::Ok().json("Logged out")
}

#[get("/me")]
async fn get_user(id: Identity, ctx: web::Data<Arc<ServerContext>>) -> impl Responder {
    let uuid = Uuid::parse_str(&id.id().unwrap()).unwrap();
    let user = ctx.users_service.find_by_uuid(uuid).await;

    match user {
        Ok(user) => HttpResponse::Ok().json(UserResponse::from(user)),
        Err(ServiceError::NotFound) => HttpResponse::NotFound().json("User not found".to_string()),
        Err(ServiceError::Forbidden) => HttpResponse::Forbidden().json("Forbidden".to_string()),
        Err(ServiceError::Other(e)) => HttpResponse::InternalServerError().json(e),
        Err(_) => HttpResponse::InternalServerError().json("Error".to_string()),
    }
}

#[patch("/me")]
async fn update_user(
    update_user: web::Json<UpdateUser>,
    id: Identity,
    ctx: web::Data<Arc<ServerContext>>,
) -> impl Responder {
    let uuid = Uuid::parse_str(&id.id().unwrap()).unwrap();
    let update_user = update_user.into_inner();

    match update_user.validate() {
        Ok(_) => (),
        Err(e) => return HttpResponse::BadRequest().json(e),
    }

    let user = ctx.users_service.update(uuid, update_user).await;

    match user {
        Ok(user) => HttpResponse::Ok().json(UserResponse::from(user)),
        Err(ServiceError::NotUnique(e)) => HttpResponse::Conflict().json(e),
        Err(_) => HttpResponse::InternalServerError().json("Error".to_string()),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(register)
            .service(login)
            .service(logout)
            .service(get_user)
            .service(update_user),
    );
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            user_uuid: user.user_uuid,
            email: user.email,
            username: user.username,
        }
    }
}

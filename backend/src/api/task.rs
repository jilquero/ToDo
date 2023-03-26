use std::sync::Arc;

use actix_identity::Identity;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use common::model::task::{
    CreateTask, Task as TaskResponse, TaskState as TaskStateResponse, UpdateTask,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    domain::{
        error::Error as ServiceError,
        tasks::task::{Task, TaskState},
    },
    ServerContext,
};

#[get("")]
async fn get_tasks(id: Identity, ctx: web::Data<Arc<ServerContext>>) -> impl Responder {
    let user_uuid = Uuid::parse_str(&id.id().unwrap()).unwrap();

    let tasks = ctx.tasks_service.find_tasks_by_user(user_uuid).await;

    match tasks {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(ServiceError::Other(e)) => HttpResponse::InternalServerError().json(e),
        Err(_) => HttpResponse::InternalServerError().json("Error".to_string()),
    }
}

#[post("")]
async fn create_task(
    create_task: web::Json<CreateTask>,
    id: Identity,
    ctx: web::Data<Arc<ServerContext>>,
) -> impl Responder {
    let user_uuid = Uuid::parse_str(&id.id().unwrap()).unwrap();
    let create_task = create_task.into_inner();

    match create_task.validate() {
        Ok(_) => (),
        Err(e) => return HttpResponse::BadRequest().json(e),
    }

    let task = ctx.tasks_service.save(user_uuid, create_task).await;

    match task {
        Ok(task) => HttpResponse::Ok().json(TaskResponse::from(task)),
        Err(ServiceError::Other(e)) => HttpResponse::InternalServerError().json(e),
        Err(_) => HttpResponse::InternalServerError().json("Error".to_string()),
    }
}

#[get("/{task_uuid}")]
async fn get_task(
    uuid: web::Path<Uuid>,
    id: Identity,
    ctx: web::Data<Arc<ServerContext>>,
) -> impl Responder {
    let user_uuid = Uuid::parse_str(&id.id().unwrap()).unwrap();
    let task_uuid = uuid.into_inner();
    let task = ctx
        .tasks_service
        .find_task_by_user(user_uuid, task_uuid)
        .await;

    match task {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(ServiceError::Forbidden) => HttpResponse::Forbidden().json("Forbidden".to_string()),
        Err(ServiceError::NotFound) => {
            HttpResponse::NotFound().json(format!("Task {} not found", task_uuid))
        }
        Err(ServiceError::Other(e)) => HttpResponse::InternalServerError().json(e),
        Err(_) => HttpResponse::InternalServerError().json("Error".to_string()),
    }
}

#[patch("/{task_uuid}")]
async fn update_task(
    update_task: web::Json<UpdateTask>,
    uuid: web::Path<Uuid>,
    id: Identity,
    ctx: web::Data<Arc<ServerContext>>,
) -> impl Responder {
    let user_uuid = Uuid::parse_str(&id.id().unwrap()).unwrap();
    let task_uuid = uuid.into_inner();
    let update_task = update_task.into_inner();

    match update_task.validate() {
        Ok(_) => (),
        Err(e) => return HttpResponse::BadRequest().json(e),
    }

    let task = ctx
        .tasks_service
        .update(user_uuid, task_uuid, update_task)
        .await;

    match task {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(ServiceError::Forbidden) => HttpResponse::Forbidden().json("Forbidden".to_string()),
        Err(ServiceError::NotFound) => {
            HttpResponse::NotFound().json(format!("Task {} not found", task_uuid))
        }
        Err(ServiceError::Other(e)) => HttpResponse::InternalServerError().json(e),
        Err(_) => HttpResponse::InternalServerError().json("Error2".to_string()),
    }
}

#[delete("/{task_uuid}")]
async fn delete_task(
    uuid: web::Path<Uuid>,
    id: Identity,
    ctx: web::Data<Arc<ServerContext>>,
) -> impl Responder {
    let user_uuid = Uuid::parse_str(&id.id().unwrap()).unwrap();
    let task_uuid = uuid.into_inner();
    let res = ctx.tasks_service.delete(user_uuid, task_uuid).await;

    match res {
        Ok(_) => HttpResponse::Ok().json("Deleted".to_string()),
        Err(ServiceError::Forbidden) => HttpResponse::Forbidden().json("Forbidden".to_string()),
        Err(ServiceError::NotFound) => {
            HttpResponse::NotFound().json(format!("Task {} not found", task_uuid))
        }
        Err(ServiceError::Other(e)) => HttpResponse::InternalServerError().json(e),
        Err(_) => HttpResponse::InternalServerError().json("Error3".to_string()),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tasks")
            .service(get_tasks)
            .service(get_task)
            .service(create_task)
            .service(update_task)
            .service(delete_task),
    );
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        Self {
            user_uuid: task.user_uuid,
            task_uuid: task.task_uuid,
            title: task.title,
            state: TaskStateResponse::from(task.state),
            description: task.description,
        }
    }
}

impl From<TaskState> for TaskStateResponse {
    fn from(task: TaskState) -> Self {
        match task {
            TaskState::NotStarted => TaskStateResponse::NotStarted,
            TaskState::InProgress => TaskStateResponse::InProgress,
            TaskState::Completed => TaskStateResponse::Completed,
        }
    }
}

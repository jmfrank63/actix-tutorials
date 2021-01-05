use crate::models::Status;
use crate::db;
use actix_web::{web, Responder, HttpResponse};
use deadpool_postgres::{Client, Pool};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status { status: "Ok".to_string() })
}

pub async fn hello() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status { status: "Hello!".to_string() })
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {

    let client: Client = 
        db_pool.get().await.expect("Error conneting to the database");

    let result = db::get_todos(&client).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
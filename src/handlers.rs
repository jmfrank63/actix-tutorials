use crate::models::{Status, CreateTodoList, CheckedResponse};
use crate::db;
use actix_web::{web, Responder, HttpResponse};
use deadpool_postgres::{Client, Pool};
use std::io::ErrorKind::Other;

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

pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {

    let client: Client = 
        db_pool.get().await.expect("Error conneting to the database");

    let result = db::get_items(&client, path.0).await;

    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn create_todo(db_pool: web::Data<Pool>, json: web::Json<CreateTodoList>) -> impl Responder {

    let client: Client = 
        db_pool.get().await.expect("Error conneting to the database");

    let result = db::create_todo(&client, json.title.clone()).await;

    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn check_item(db_pool: web::Data<Pool>, path: web::Path<(i32, i32)>) -> impl Responder {

    let client: Client = 
        db_pool.get().await.expect("Error conneting to the database");

    let result = db::check_item(&client, path.0, path.1).await;

    match result {
        Ok(()) => HttpResponse::Ok().json(CheckedResponse{ success: true}),
        Err(ref e) if e.kind() == Other => HttpResponse::Ok().json(CheckedResponse{success: false}),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
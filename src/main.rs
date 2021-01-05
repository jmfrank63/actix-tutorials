mod config;
mod models;
mod handlers;
mod db;

use actix_web::{HttpServer, App, web};
use std::io::Result;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use handlers::{status, hello, get_todos, get_items, create_todo, check_item};

#[actix_rt::main]
async fn main() -> Result<()> {

    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("Starting web server at http://{}:{}", config.server.host, config.server.port);

    let server_res = HttpServer::new(move || {

        App::new()
            .data(pool.clone())
            .route("/", web::get().to(status))
            .route("/hello", web::get().to(hello))
            .route("/todos{_:/?}", web::get().to(get_todos))
            .route("/todos{_:/?}", web::post().to(create_todo))
            .route("/todos/{list_id}/items{_:/?}", web::get().to(get_items))
            .route("/todos/{list_id}/items/{item_id}{_:/?}", web::put().to(check_item))
            
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await?;
    Ok(server_res)
}

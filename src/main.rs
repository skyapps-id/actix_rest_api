use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

mod handlers;
mod pkg;
use handlers::models::*;  // Update to use the correct path

type Db = Mutex<Vec<Book>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = web::Data::new(Db::new(Vec::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .configure(handlers::routes::configure)  // Update to use the correct path
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

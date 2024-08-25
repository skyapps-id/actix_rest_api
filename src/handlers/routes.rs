use actix_web::web;
use crate::handlers::handlers::*;  // Update to use the correct path

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/books")
            .route(web::get().to(get_books))
            .route(web::post().to(create_book)),
    )
    .service(
        web::resource("/books/{id}")
            .route(web::get().to(get_book))
            .route(web::put().to(update_book))
            .route(web::delete().to(delete_book)),
    );
}

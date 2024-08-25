use actix_web::{web, HttpResponse, Responder, http::StatusCode};
use std::sync::Mutex;
use uuid::Uuid; 

use crate::pkg::response::{success, fail};
use crate::handlers::models::Book;

type Db = Mutex<Vec<Book>>;

pub async fn get_books(db: web::Data<Db>) -> impl Responder {
    let books = db.lock().unwrap();
    success(books.clone())
}

pub async fn get_book(db: web::Data<Db>, book_id: web::Path<Uuid>) -> impl Responder {
    let books = db.lock().unwrap();
    match books.iter().find(|&book| book.id == Some(*book_id)) {
        Some(book) => success(book.clone()),
        None => fail("Book not found", StatusCode::NOT_FOUND),
    }
}

pub async fn create_book(db: web::Data<Db>, new_book: web::Json<Book>) -> impl Responder {
    let mut books = db.lock().unwrap();
    let mut book = new_book.into_inner();
    book.id = Some(Uuid::new_v4());
    books.push(book.clone());
    HttpResponse::Created().json(book)
}

pub async fn update_book(
    db: web::Data<Db>,
    book_id: web::Path<Uuid>,
    updated_book: web::Json<Book>,
) -> impl Responder {
    let mut books = db.lock().unwrap();
    match books.iter_mut().find(|book| book.id == Some(*book_id)) {
        Some(book) => {
            book.title = updated_book.title.clone();
            book.author = updated_book.author.clone();
            book.published_year = updated_book.published_year;
            HttpResponse::Ok().json(book)
        }
        None => HttpResponse::NotFound().finish(),
    }
}

pub async fn delete_book(db: web::Data<Db>, book_id: web::Path<Uuid>) -> impl Responder {
    let mut books = db.lock().unwrap();
    if let Some(pos) = books.iter().position(|book| book.id == Some(*book_id)) {
        books.remove(pos);
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

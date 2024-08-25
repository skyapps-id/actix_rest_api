use actix_web::{HttpResponse, http::StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: T,
}

pub fn success<T: Serialize>(data: T) -> HttpResponse {
    let response = ApiResponse {
        success: true,
        message: "success".to_string(),
        data,
    };
    HttpResponse::Ok().json(response)
}

pub fn fail(message: &str, status_code: StatusCode) -> HttpResponse {
    let response = ApiResponse {
        success: false,
        message: message.to_string(),
        data: (),
    };
    HttpResponse::build(status_code).json(response)
}
use crate::models::Status;
use crate::models::PersonDto;
use actix_web::{web, Responder};

pub async fn status() -> impl Responder{
    web::Json(Status {status: "ok".to_string() })
}

pub async fn get_person() -> impl Responder{
    web::Json(PersonDto {
        first_name: "Rakhmon".to_string(),
        last_name: "Radjabov".to_string(),
        email: "commelevent@protonmail.com".to_string(),
    })
}
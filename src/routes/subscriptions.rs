use actix_web::{web, HttpResponse};
use sqlx::PgPool;


#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}


pub async fn subscribe(
    _form: web::Form<FormData>,
    _pool: web::Data<PgPool>
) -> HttpResponse {

    match sqlx::query!("")
        .execute(_pool.as_ref())
        .await
        {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            return HttpResponse::InternalServerError().finish()
        }
    }
}
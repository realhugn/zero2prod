use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use sqlx::types::Uuid;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}


pub async fn subscribe(
    _form: web::Form<FormData>,
    _pool: web::Data<PgPool>
) -> HttpResponse {    
    
    let request_id = Uuid::new_v4();

    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %_form.email,
        subscriber_name= %_form.name
    );
    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!(
        "Saving new subscriber details in the database"
    );
    
    match sqlx::query!(r#"
        insert into subscriptions (id, email, name, subscribed_at) 
            values ($1, $2, $3, $4);
        "#, 
        Uuid::new_v4(),
        _form.email,
        _form.name, 
        Utc::now()
        )
        .execute(_pool.as_ref())
        .instrument(query_span)
        .await
        {
        Ok(_) => {
            tracing::info!("request_id {} - New subscriber details have been saved",request_id);
            HttpResponse::Ok().body("Thanks you for your subscription")
        },
        Err(e) => {
            tracing::error!("Frequest_id {} - Failed to execute query: {:?}",request_id, e);
            return HttpResponse::InternalServerError().finish()
        }
    }
}
use crate::common::configuration::database::postgres_config::PgPool;
use crate::models::subscription::dtos::create_subscription::SubscribeDto;
use crate::models::subscription::Subscription;
use crate::schema::subscriptions::dsl;
use actix_web::{web, HttpResponse};
use diesel::insert_into;
use diesel::RunQueryDsl;

pub async fn subscribe(_body: web::Json<SubscribeDto>, pool: web::Data<PgPool>) -> HttpResponse {
    let conn = pool.get();
    if let Err(e) = conn {
        println!("Error: {}", e);
        return HttpResponse::InternalServerError().finish();
    }
    let new_subscription: Subscription = _body.into_inner().into();
    let mut conn = conn.unwrap();
    match web::block(move || {
        insert_into(dsl::subscriptions)
            .values(&new_subscription)
            .execute(&mut conn)
    })
    .await
    {
        Ok(query_result) => match query_result {
            Ok(result) => {
                println!("Subscription saved, {:#?}", result);
                HttpResponse::Ok().finish()
            }
            Err(e) => {
                println!("Error: {}", e);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(e) => {
            println!("Error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

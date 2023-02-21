use crate::common::configuration::database::postgres_config::PgPool;
use crate::schema::subscriptions::dsl;
use actix_web::{web, HttpResponse};
use diesel::insert_into;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SubscribeBody {
    name: String,
    email: String,
}

impl SubscribeBody {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn new(name: String, email: String) -> Self {
        Self { name, email }
    }
}

pub async fn subscribe(_body: web::Json<SubscribeBody>, pool: web::Data<PgPool>) -> HttpResponse {
    let conn = pool.get();
    if let Err(e) = conn {
        println!("Error: {}", e);
        return HttpResponse::InternalServerError().finish();
    }
    let mut conn = conn.unwrap();
    match web::block(move || {
        insert_into(dsl::subscriptions)
            .values((
                dsl::name.eq(_body.name()),
                dsl::email.eq(_body.email()),
                dsl::subscribed_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(&mut conn)
    })
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

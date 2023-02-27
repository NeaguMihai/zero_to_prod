use crate::common::configuration::database::postgres_config::PgPool;
use crate::models::subscription::dtos::create_subscription::SubscribeDto;
use crate::models::subscription::Subscription;
use crate::schema::subscriptions::dsl;
use actix_web::{web, HttpResponse};
use diesel::insert_into;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
use diesel::PgConnection;
use diesel::RunQueryDsl;
use r2d2::PooledConnection;

#[tracing::instrument(
    name = "Adding a new subscriber.",
    skip(body, pool),
    fields(
        subscriber_email = %body.email(),
        subscriber_name = %body.name()
    )
)]
pub async fn subscribe(body: web::Json<SubscribeDto>, pool: web::Data<PgPool>) -> HttpResponse {
    let conn = pool.get().unwrap();
    log::info!("Saving subscription");

    let new_subscription: Subscription = body.into_inner().into();
    match web::block(move || insert_new_subscriber(conn, new_subscription)).await {
        Ok(save_result) => match save_result {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => {
                log::error!("Error: {}", e);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(e) => {
            log::error!("Error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(name = "Inserting subscriber.", skip(conn, new_subscription))]
fn insert_new_subscriber(
    mut conn: PooledConnection<ConnectionManager<PgConnection>>,
    new_subscription: Subscription,
) -> Result<(), Error> {
    match insert_into(dsl::subscriptions)
        .values(&new_subscription)
        .execute(&mut conn)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

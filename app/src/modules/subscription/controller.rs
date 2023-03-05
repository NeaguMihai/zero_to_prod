use crate::common::configuration::database::postgres_config::PgPool;
use crate::models::subscription::dtos::create_subscription::SubscribeDto;
use crate::models::subscription::Subscription;
use crate::schema::subscriptions::dsl;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use macros::controller;
use r2d2::PooledConnection;
use macros::RouteController;
use app_core::traits::Controller;


#[controller("/subscriptions")]
pub struct SubscriptionController {}

// #[tracing::instrument(
//     name = "Adding a new subscriber.",
//     skip(body, pool),
//     fields(
//         subscriber_email = %body.email(),
//         subscriber_name = %body.name()
//     )
// )]
pub async fn subscribe(
    axum::Json(body): axum::Json<SubscribeDto>,
    pool: Extension<PgPool>,
) -> Response {
    // let conn = pool.get().unwrap();
    // log::info!("Saving subscription");

    // let new_subscription: Subscription = body.into();

    // match insert_new_subscriber(conn, new_subscription) {
    //     Ok(save_result) => Json(save_result).into_response(),
    //     Err(e) => {
    //         log::error!("Error: {}", e);
    //         (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e)).into_response()
    //     }
    // }
    ().into_response()
}

// #[tracing::instrument(name = "Inserting subscriber.", skip(conn, new_subscription))]
fn insert_new_subscriber(
    mut conn: PooledConnection<ConnectionManager<PgConnection>>,
    new_subscription: Subscription,
) -> Result<Subscription, diesel::result::Error> {
    // match insert_into(dsl::subscriptions)
    //     .values(&new_subscription)
    // {
    //     Ok(save_result) => Ok(save_result),
    //     Err(e) => Err(e),
    // }
    Ok(new_subscription)
}

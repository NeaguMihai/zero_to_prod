use crate::common::configuration::database::postgres_config::PgPool;
use crate::models::subscription::dtos::create_subscription::SubscribeDto;
use crate::models::subscription::Subscription;
use crate::schema::subscriptions::dsl::*;
use app_core::traits::Controller;
use axum::response::{Response, IntoResponse};
use axum::routing::Router;
use axum::{Extension, Json};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::{insert_into, PgConnection};
use hyper::StatusCode;
use macros::{controller, post, RouteController};
use r2d2::PooledConnection;

#[tracing::instrument(
    name = "Adding a new subscriber.",
    skip(body, pool),
    fields(
        subscriber_email = %body.email(),
        subscriber_name = %body.name()
    )
)]
#[post("subscribe")]
pub async fn subscribe(pool: Extension<PgPool>, Json(body): Json<SubscribeDto>) -> Response {
    let conn = pool.get().unwrap();
    log::info!("Saving subscription");

    let new_subscription: Subscription = body.into();

    match insert_new_subscriber(conn, new_subscription) {
        Ok(save_result) => Json(save_result).into_response(),
        Err(e) => {
            log::error!("Error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e)).into_response()
        }
    }
}

fn insert_new_subscriber(
    mut conn: PooledConnection<ConnectionManager<PgConnection>>,
    new_subscription: Subscription,
) -> Result<Subscription, diesel::result::Error> {
    let insert_result = insert_into(subscriptions)
        .values(&new_subscription)
        .get_result(&mut conn);
    match insert_result {
        Ok(save_result) => return Ok(save_result),
        Err(e) => return Err(e),
    };
}

#[controller("subscriptions")]
pub struct SubscriptionController {}
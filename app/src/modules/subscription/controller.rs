use crate::common::configuration::database::postgres_config::PgPool;
use crate::common::core::traits::Controller;
use crate::models::subscription::dtos::create_subscription::SubscribeDto;
use crate::models::subscription::Subscription;
use crate::schema::subscriptions::dsl;
use axum::http::Error;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use axum::Json;
use diesel::insert_into;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use diesel::RunQueryDsl;
use r2d2::PooledConnection;

pub struct SubscriptionController {}

impl SubscriptionController {
    fn name(&self) -> &'static str {
        "SubscriptionController"
    }

    fn base_path(&self) -> &'static str {
        "/subscriptions"
    }

    fn register_routes<S, B>(
        &self,
    ) -> Vec<(
        String,
        axum::routing::MethodRouter<S, B, std::convert::Infallible>,
    )>
    where
        B: axum::body::HttpBody + Send + Sync + 'static,
        S: Clone + Send + Sync + 'static,
    {
        vec![]
    }
}

// #[tracing::instrument(
//     name = "Adding a new subscriber.",
//     skip(body, pool),
//     fields(
//         subscriber_email = %body.email(),
//         subscriber_name = %body.name()
//     )
// )]
pub async fn subscribe(
    axum::Json(body): axum::Json<SubscribeDto>, pool: Extension<PgPool>
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

#[tracing::instrument(name = "Inserting subscriber.", skip(conn, new_subscription))]
fn insert_new_subscriber(
    mut conn: PooledConnection<ConnectionManager<PgConnection>>,
    new_subscription: Subscription,
) -> Result<Subscription, diesel::result::Error> {
    match insert_into(dsl::subscriptions)
        .values(&new_subscription)
        .get_result(&mut conn)
    {
        Ok(save_result) => Ok(save_result),
        Err(e) => Err(e),
    }
}

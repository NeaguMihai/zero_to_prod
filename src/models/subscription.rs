use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Subscription {
    id: Uuid,
    email: String,
    name: String,
    subscribed_at: NaiveDateTime,
}

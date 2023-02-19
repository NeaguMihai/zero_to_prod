use diesel::{Queryable};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Subscription {
    id: Uuid,
    email: String,
    name: String,
    subscribed_at: NaiveDateTime,
}
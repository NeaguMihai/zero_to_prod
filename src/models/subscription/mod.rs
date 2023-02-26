pub mod dtos;

use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::subscriptions;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = subscriptions)]
pub struct Subscription {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub subscribed_at: NaiveDateTime,
}

impl Subscription {
    pub fn new(email: String, name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            name,
            subscribed_at: chrono::Utc::now().naive_utc(),
        }
    }
}

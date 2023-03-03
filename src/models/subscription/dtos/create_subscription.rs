use serde::{Deserialize, Serialize};

use crate::models::subscription::Subscription;

#[derive(Deserialize, Serialize, Clone)]
pub struct SubscribeDto {
    name: String,
    email: String,
}

impl SubscribeDto {
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

impl From<SubscribeDto> for Subscription {
    fn from(val: SubscribeDto) -> Self {
        Subscription::new(val.email, val.name)
    }
}

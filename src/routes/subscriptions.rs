use actix_web::{web, HttpResponse};
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

pub async fn subscribe(_body: web::Json<SubscribeBody>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

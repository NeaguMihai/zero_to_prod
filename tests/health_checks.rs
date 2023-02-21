#[cfg(test)]
mod tests {
    use diesel::prelude::*;
    use diesel::query_dsl::QueryDsl;
    use diesel::RunQueryDsl;
    use std::net::TcpListener;
    use zero_to_prod::models::subscription::Subscription;
    use zero_to_prod::schema::subscriptions::dsl::{name, subscriptions};
    use zero_to_prod::{
        common::configuration::database::DatabaseConnectionFactory, routes::SubscribeBody,
    };

    #[tokio::test]
    async fn health_check_works() {
        let address = spawn_app();
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{address}/health-check"))
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
    }

    #[tokio::test]
    async fn subscribe_returns_a_200_for_valid_form_data() {
        let address = spawn_app();

        let db_connection = &mut DatabaseConnectionFactory::get_pg_connection_pool()
            .expect("Failed to connect to database");

        let client = reqwest::Client::new();

        let body = SubscribeBody::new("Ursula Le Guin".to_string(), "em@mail.com".to_string());
        let response = client
            .post(format!("{address}/subscribe"))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&body).unwrap())
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(200, response.status().as_u16());
        let mut conn = db_connection
            .get()
            .expect("Failed to get connection from pool");
        let _saved = subscriptions
            .filter(name.eq("adas"))
            .load::<Subscription>(&mut conn)
            .expect("Failed to load subscriptions from database");
    }

    #[tokio::test]
    async fn subscribe_returns_a_400_for_invalid_form_data() {
        let address = spawn_app();
        let client = reqwest::Client::new();
        let test_cases = vec![
            (r#"{"email":"mimi@mail.com"}"#, "missing the `name` field"),
            (r#"{"name":"name"}"#, "missing the `email` field"),
            (r#"{}"#, "missing both `name` and `email` fields"),
        ];
        for (invalid_body, error_message) in test_cases {
            let response = client
                .post(&format!("{address}/subscribe"))
                .header("Content-Type", "application/json")
                .body(invalid_body)
                .send()
                .await
                .expect("Failed to execute request.");

            assert_eq!(
                400,
                response.status().as_u16(),
                // Additional customised error message on test failure
                "The API did not fail with 400 Bad Request when the payload was {}.",
                error_message
            );
        }
    }

    fn spawn_app() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let db_connection = DatabaseConnectionFactory::get_pg_connection_pool()
            .expect("Failed to connect to database");
        let server = zero_to_prod::run(listener, db_connection).expect("Failed to bind address");
        let _ = tokio::spawn(server);
        format!("http://127.0.0.1:{port}")
    }
}

#[cfg(test)]
mod tests {
    use app::common::configuration::database::DatabaseConnectionFactory;
    use app::common::configuration::database::DatabaseConnectionOptions;
    use app::common::configuration::database::postgres_config::PgPool;
    use app::common::configuration::database::run_migrations;
    use app::common::configuration::logger::setup_logger;
    use app::models::subscription::Subscription;
    use app::models::subscription::dtos::create_subscription::SubscribeDto;
    use app::schema::subscriptions::dsl::subscriptions;
    use app::startup::run;
    use diesel::pg::Pg;
    use diesel::sql_query;
    use diesel::RunQueryDsl;
    use once_cell::sync::Lazy;
    use std::net::TcpListener;
    use uuid::Uuid;

    static TRACING: Lazy<()> = Lazy::new(|| {
        if std::env::var("TEST_LOG").is_ok() {
            setup_logger(std::io::stdout)
        } else {
            setup_logger(std::io::sink)
        }
    });

    struct TestApp {
        app_url: String,
        pg_connection_pool: PgPool,
    }

    #[tokio::test]
    async fn health_check_works() {
        let app = spawn_app();
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/health-check", app.app_url))
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
    }

    #[tokio::test]
    async fn subscribe_returns_a_200_for_valid_form_data() {
        let app = spawn_app();

        let client = reqwest::Client::new();

        let body = SubscribeDto::new("Ursula Le Guin".to_string(), "em@mail.com".to_string());
        let response = client
            .post(format!("{}/subscribe", app.app_url))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&body).unwrap())
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            200,
            response.status().as_u16(),
            "The API did not return a 200 OK status code when the payload was valid."
        );

        let mut conn = app
            .pg_connection_pool
            .get()
            .expect("Failed to get connection from pool");

        let saved = subscriptions
            .load::<Subscription>(&mut conn)
            .expect("Failed to load subscriptions from database");

        assert_eq!(1, saved.len(), "Expected one subscription in the database.");
        let saved = saved.get(0).expect("Failed to get first subscription");

        assert_eq!(
            body.name(),
            saved.name,
            "Expected subscription name to be {} but was {}",
            body.name(),
            saved.name
        );
        assert_eq!(
            body.email(),
            saved.email,
            "Expected subscription email to be {} but was {}",
            body.email(),
            saved.email
        );
    }

    #[tokio::test]
    async fn subscribe_returns_a_400_for_invalid_form_data() {
        let app = spawn_app();
        let client = reqwest::Client::new();
        let test_cases = vec![
            (r#"{"email":"mimi@mail.com"}"#, "missing the `name` field"),
            (r#"{"name":"name"}"#, "missing the `email` field"),
            (r#"{}"#, "missing both `name` and `email` fields"),
        ];
        for (invalid_body, error_message) in test_cases {
            let response = client
                .post(&format!("{}/subscribe", app.app_url))
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

    fn spawn_app() -> TestApp {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let conn = db_bootstrap();
        let server = run(listener, conn.clone());
        let _ = tokio::spawn(server);
        TestApp {
            app_url: format!("http://0.0.0.0:{port}"),
            pg_connection_pool: conn,
        }
    }
    fn db_bootstrap() -> PgPool {
        Lazy::force(&TRACING);
        let connection_options = DatabaseConnectionOptions::default();
        let mut db_connection = DatabaseConnectionFactory::get_pg_connection(connection_options);
        let db_name = format!("db_{}", Uuid::new_v4().to_string().replace("-", ""));

        println!("Creating database: {}", db_name);
        sql_query(format!("CREATE DATABASE {}", db_name))
            .execute(&mut db_connection)
            .expect("Failed to create database");

        let mut connection_options = DatabaseConnectionOptions::default();
        connection_options.database = Some(db_name.clone());
        let db_connection = DatabaseConnectionFactory::get_pg_connection_pool(connection_options)
            .expect("Failed to connect to database");
        run_migrations::<Pg>(&mut db_connection.get().expect("Failed to run migrations"));

        db_connection
    }
}

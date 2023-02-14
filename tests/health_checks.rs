#[cfg(test)]
mod tests {
    use std::net::TcpListener;

    #[tokio::test]
    async fn health_check_works() {
        let address = spawn_app();
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{address}/health_check"))
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
        let client = reqwest::Client::new();

        let body = r#"{"name":"derek","email"="derek@example.com"}"#;
        let response = client
            .post(format!("{address}/subscriptions"))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(200, response.status().as_u16());
    }

    #[tokio::test]
    async fn subscribe_returns_a_400_for_invalid_form_data() {
        let address = spawn_app();
        let client = reqwest::Client::new();
        let test_cases = vec![
            ("name=le%20guin", "missing the email"),
            ("email=ursula_le_guin%40gmail.com", "missing the name"),
            ("", "missing both name and email"),
        ];
        for (invalid_body, error_message) in test_cases {
            let response = client
                .post(&format!("{address}/subscriptions"))
                .header("Content-Type", "application/x-www-form-urlencoded")
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
        let server = zero_to_prod::run(listener).expect("Failed to bind address");
        let _ = tokio::spawn(server);
        format!("http://127.0.0.1:{port}")
    }
}

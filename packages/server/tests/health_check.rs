use backlogger_server::server::{config::Config, router::Router, Server};

#[tokio::test]
async fn health_check_works() {
    let config = Config::new("127.0.0.1", 3000);
    tokio::spawn(Server::new(config.clone(), Router::default()).run());

    let client = reqwest::Client::new();
    let url = format!("http://{}:{}/health_check", config.host(), config.port());

    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

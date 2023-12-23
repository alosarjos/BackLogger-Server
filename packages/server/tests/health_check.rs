use backlogger_server::server::{config::Config, router::Router, Server};

#[tokio::test]
async fn health_check_works() {
    let config: Config = Config::new("127.0.0.1", 3000);
    let (server_tx, server_rx) = tokio::sync::oneshot::channel::<bool>();
    tokio::spawn(Server::new(config.clone(), Router::default()).run(Some(server_tx)));

    server_rx.await.expect("Could not receive message");

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

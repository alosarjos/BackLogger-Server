use backlogger_server::server::{config::Config, Server};

fn spawn_app(config: Config) -> tokio::task::JoinHandle<Result<(), hyper::Error>> {
    let server = Server::new(config).run();
    tokio::spawn(server)
}

#[tokio::test]
async fn health_check_works() {
    let config = Config::new("127.0.0.1", 3000);

    spawn_app(config.clone());
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

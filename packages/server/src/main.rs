use backlogger_server::server::{config::Config, Server};

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let config = Config::new("0.0.0.0", 3000);
    let server = Server::new(config);
    server.run().await
}

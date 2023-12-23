use backlogger_server::server::Server;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let server = Server::default();
    server.run(None).await
}

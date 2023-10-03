use backlogger_server::server::Server;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let server = Server::default();
    server.run().await
}

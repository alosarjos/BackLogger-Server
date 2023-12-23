use self::{config::Config, router::Router};

use tokio::net::TcpListener;

pub mod config;
pub mod controller;
pub mod router;

pub struct Server {
    pub config: Config,
    pub router: Router,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            config: Default::default(),
            router: Default::default(),
        }
    }
}

impl Server {
    pub fn new(config: Config, router: Router) -> Self {
        Self {
            config,
            router,
        }
    }

    pub async fn run(self, notifier: Option<tokio::sync::oneshot::Sender<bool>>) -> Result<(), std::io::Error> {
        let address = format!("{}:{}", self.config.host(), self.config.port());
        let listener = TcpListener::bind(address).await?;
        let server = axum::serve(listener, self.router.get_as_service());

        if let Some(notifier) = notifier {
            notifier.send(true).expect("Could not send the message");
        }

        server.await.unwrap();
        Ok(())
    }
}

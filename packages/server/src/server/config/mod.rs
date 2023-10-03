#[derive(Clone, Debug)]
pub struct Config {
    host: String,
    port: usize,
}

impl Config {
    pub fn new(host: &str, port: usize) -> Self {
        Self {
            host: host.to_string(),
            port,
        }
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> usize {
        self.port
    }
}

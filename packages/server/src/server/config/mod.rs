#[derive(Clone, Debug)]
pub struct Config {
    host: String,
    port: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: String::from("0.0.0.0"),
            port: 3000,
        }
    }
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

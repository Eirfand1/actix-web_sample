pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn new() -> Self {
        println!("server running at http://127.0.0.1:8080");
        Self {
            host: String::from("127.0.0.1"),
            port: 8080,
        }
    }
}

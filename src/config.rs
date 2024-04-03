use std::env;

use dotenvy::dotenv;
use tokio::sync::OnceCell;

struct ServerConfig {
    host: String,
    port: u16,
}

struct DatabaseConfig {
    url: String,
}

pub struct Config {
    server: ServerConfig,
    db: DatabaseConfig,
}

impl Config {
    pub fn db_url(&self) -> &str {
        &self.db.url
    }

    pub fn server_host(&self) -> &str {
        &self.server.host
    }

    pub fn server_port(&self) -> u16 {
        self.server.port
    }
}

async fn init_config() -> Config {
    dotenv().ok();

    let server_config = ServerConfig {
        host: env::var("HOST").unwrap_or_else(|_| String::from("0.0.0.0")),
        port: env::var("PORT")
            .unwrap_or_else(|_| String::from("3000"))
            .parse()
            .expect("Incorrect port value, expected u16"),
    };

    let database_config = DatabaseConfig {
        url: env::var("DATABASE_URL").expect("not found env DATABASE_URL"),
    };
    
    Config {
        server: server_config,
        db: database_config,
    }
}

pub static CONFIG: OnceCell<Config> = OnceCell::const_new();

pub async fn config() -> &'static Config {
    CONFIG.get_or_init(init_config).await
}
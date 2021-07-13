use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storage {
    pub images: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiServer {
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub access_token_secret: String,
    pub refresh_token_secret: String,
    pub access_token_expiration_time: u32,
    pub refresh_token_expiration_time: u32,
    pub root_user_email: String,
    pub root_user_password: String,
    pub database: Database,
    pub storage: Storage,
    pub server: ApiServer
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        let mut c = config::Config::default();

        // Default
        let default = config::Config::try_from(&Config::default())?;
        c.merge(default)?;

        // Merge config file
        if cfg!(debug_assertions) {
            c.merge(config::File::with_name("config/debug").required(false))?;
        } else {
            c.merge(config::File::with_name("config/release").required(false))?;
        }

        // Merge environment
        c.merge(config::Environment::with_prefix("APP"))?;

        c.try_into()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            access_token_secret: "secret".to_string(),
            refresh_token_secret: "secret".to_string(),
            access_token_expiration_time: 3600,
            refresh_token_expiration_time: 31536000,
            root_user_email: "root@taskrs.com".to_string(),
            root_user_password: "root".to_string(),
            database: Database {
                user: "postgres".to_string(),
                password: "password".to_string(),
                host: "localhost".to_string(),
                port: 5432,
                database: "taskrs".to_string()
            },
            storage: Storage {
                images: "storage/images".to_string()
            },
            server: ApiServer {
                address: "0.0.0.0".to_string(),
                port: 8080
            },
        }
    }
}
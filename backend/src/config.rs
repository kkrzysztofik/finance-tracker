use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub bind_addr: String,
    pub auth_user: String,
    pub auth_pass: String,
    pub openai_api_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://finance:finance@localhost:5432/finance".into()),
            bind_addr: env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3001".into()),
            auth_user: env::var("AUTH_USER").unwrap_or_else(|_| "admin".into()),
            auth_pass: env::var("AUTH_PASS").unwrap_or_else(|_| "admin".into()),
            openai_api_key: env::var("OPENAI_API_KEY").ok(),
        }
    }
}

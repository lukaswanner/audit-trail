use sqlx::{
    postgres::{PgConnectOptions, PgSslMode},
    PgPool,
};

struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl DatabaseSettings {
    fn create_pool(&self) -> PgPool {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        let options = PgConnectOptions::new()
            .username(&self.username)
            .password(&self.password)
            .host(&self.host)
            .port(self.port)
            .database(&self.database_name)
            .ssl_mode(ssl_mode);

        let pool = PgPool::connect_lazy_with(options);
        return pool;
    }
}

impl Database {
    pub fn new_localhost() -> Database {
        let settings = DatabaseSettings {
            username: String::from("postgres"),
            password: String::from("password"),
            port: 5432,
            host: String::from("localhost"),
            database_name: String::from("audit_trail"),
            require_ssl: false,
        };
        let pool = settings.create_pool();
        return Database { pool };
    }

    pub fn generate_api_token(&self) -> String {
        return String::from("token");
    }
}

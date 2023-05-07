//! src/configuration.rs

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub port: u16
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.database_name
        )
    }
}


pub fn get_config() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        // Adding configuration file as source
        .add_source(config::File::with_name("configuration"))
        .build()
        .unwrap();

    settings.try_deserialize()
}

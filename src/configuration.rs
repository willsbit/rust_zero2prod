use config::{Config, ConfigError, File, FileFormat};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    // initialize config reader
    let settings = Config::builder()
        .set_default("default", "1")?
        .add_source(File::new("configuration", FileFormat::Yaml))
        .build()?;

    settings.try_deserialize::<Settings>()
}

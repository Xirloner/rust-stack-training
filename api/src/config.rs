use config::{Config, ConfigError, Environment, File};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_url: String,
    //pub api_key: String,
    //#[serde(default = "default_log_level")]
    //pub log_level: String,
}

//fn default_log_level() -> String {
//   "info".to_string()
//}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok(); // Load from `.env`

        let builder = Config::builder()
            .add_source(File::with_name(".env").required(false))
            .add_source(Environment::default().separator("__"));

        builder.build()?.try_deserialize()
    }
}

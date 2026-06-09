use tryphon::Config;
use validator::Validate;

#[derive(Debug, Config, Clone, Validate)]
pub struct AppConfig {
    // server conf
    #[env("HTTP_HOST")]
    #[default("0.0.0.0")]
    #[validate(length(min = 1))]
    pub host: String,
    #[env("HTTP_PORT")]
    #[default(8080)]
    #[validate(range(min = 0, max = 65535))]
    pub port: u16,
}

pub fn load_config() -> AppConfig {
    let c = AppConfig::load();
    match c {
        Ok(cfg) => {
            if let Err(e) = cfg.validate() {
                panic!("Configuration validation failed:\n{}", e);
            }
            cfg
        }
        Err(e) => {
            panic!("Configuration loading failed: {}", e);
        }
    }
}

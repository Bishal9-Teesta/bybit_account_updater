use std::env;
use crate::structure::config::Keys;

pub fn get_keys() -> Keys {

    let application_mode: String = env::var("APPLICATION_MODE").unwrap();

    // Development
    let development_api_key: String = env::var("DEVELOPMENT_API_KEY").unwrap();
    let development_secret_key: String = env::var("DEVELOPMENT_SECRET_KEY").unwrap();

    // Production
    let production_api_key: String = env::var("PRODUCTION_API_KEY").unwrap();
    let production_secret_key: String = env::var("PRODUCTION_SECRET_KEY").unwrap();

    if application_mode == "PRODUCTION" {
        Keys {
            api_key: production_api_key,
            secret_key: production_secret_key
        }
    } else {
        Keys {
            api_key: development_api_key,
            secret_key: development_secret_key
        }
    }
}


use std::env;
use crate::structure::config::BaseUrl;

pub fn get_base_url() -> BaseUrl {

    let application_mode: String = env::var("APPLICATION_MODE").unwrap();

    // DEVELOPMENT
    let development_socket_spot: String = "wss://stream-testnet.bybit.com/v5/public/spot".to_string();
    let development_socket_perpetual: String = "wss://stream-testnet.bybit.com/v5/public/linear".to_string();
    let development_socket_inverse: String = "wss://stream-testnet.bybit.com/v5/public/inverse".to_string();
    let development_socket_option: String = "wss://stream-testnet.bybit.com/v5/public/option".to_string();
    let development_socket_private: String = "wss://stream-testnet.bybit.com/v5/private".to_string();

    // PRODUCTION
    let production_socket_spot: String = "wss://stream.bybit.com/v5/public/spot".to_string();
    let production_socket_perpetual: String = "wss://stream.bybit.com/v5/public/linear".to_string();
    let production_socket_inverse: String = "wss://stream.bybit.com/v5/public/inverse".to_string();
    let production_socket_option: String = "wss://stream.bybit.com/v5/public/option".to_string();
    let production_socket_private: String = "wss://stream.bybit.com/v5/private".to_string();

    if application_mode == "PRODUCTION" {
        BaseUrl {
            socket_spot: production_socket_spot,
            socket_perpetual: production_socket_perpetual,
            socket_inverse: production_socket_inverse,
            socket_option: production_socket_option,
            socket_private: production_socket_private
        }
    } else {
        BaseUrl {
            socket_spot: development_socket_spot,
            socket_perpetual: development_socket_perpetual,
            socket_inverse: development_socket_inverse,
            socket_option: development_socket_option,
            socket_private: development_socket_private
        }
    }
}

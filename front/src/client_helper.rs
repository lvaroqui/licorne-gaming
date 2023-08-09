use std::sync::OnceLock;

use leptos::leptos_dom::helpers::location;
use openapi::apis::configuration::Configuration;

static CONFIG: OnceLock<Configuration> = OnceLock::new();

pub fn client_config() -> &'static Configuration {
    CONFIG.get_or_init(|| {
        let origin = location().origin().expect("Could not retrieve URL origin");
        Configuration {
            base_path: format!("{}/api", origin),
            ..Default::default()
        }
    })
}

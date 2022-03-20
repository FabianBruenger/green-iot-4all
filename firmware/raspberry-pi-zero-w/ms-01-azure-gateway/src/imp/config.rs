// -----------------------------------------------------------------------------------------
//                                      Imports
// -----------------------------------------------------------------------------------------
use serde::Deserialize;

// -----------------------------------------------------------------------------------------
//                                      Env
// -----------------------------------------------------------------------------------------
static CONFIG_DEV: &str = "config/ms01config";
static CONFIG_REL: &str = "/etc/growiot/ms01config";

/// Struct which holds the device configs to log in to Azure IoT hub <config/config.toml>
#[derive(Debug, Deserialize)]
pub struct DeviceConfig {
    pub hostname: String,
    pub device_id: String,
    pub shared_access_key: String,
    pub sub_ms02_socket: String,
    pub pub_ms01_socket: String,
    pub sub_ms02_topic: String,
    pub pub_ms01_topic: String,
    pub logging_path: String,
    pub logging_path_dev: String,
    pub intervall_zmq_rx: u16,
    pub factor_azu_tx: u8,
}

/// implementation of reading in the config file
impl DeviceConfig {
    /// Create the new config struct. Depending on dev or rel env
    pub fn new() -> Result<Self, config::ConfigError> {
        // Choose path based on environment
        let path: &str;
        if cfg!(debug_assertions) {
            path = CONFIG_DEV;
        } else {
            path = CONFIG_REL;
        }

        // create default config instance
        let mut cfg = config::Config::default();

        // try to fit the config.toml
        cfg.merge(config::File::with_name(path))?;

        // fit the config
        cfg.try_into()
    }
}

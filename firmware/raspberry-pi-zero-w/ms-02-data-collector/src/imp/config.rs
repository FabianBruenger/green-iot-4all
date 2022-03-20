//-------------------------------------------------------------------------------------------------------
//                                             Imports
//-------------------------------------------------------------------------------------------------------
use serde::Deserialize;
use std::{collections::HashMap, error::Error};

//-------------------------------------------------------------------------------------------------------
//                                             Env
//-------------------------------------------------------------------------------------------------------
static CONFIG_DEV: &str = "config/ms02config";
static CONFIG_REL: &str = "/etc/growiot/ms02config";

//-------------------------------------------------------------------------------------------------------
//                                             Sensor data
//-------------------------------------------------------------------------------------------------------

/// Has map that holds all the SensorData structs read from the <config/sensor-data.json> file.
pub type SensorMap = HashMap<String, SensorData>;

/// Function of creating the SensorMap by reading in the config
pub fn create_sensor_map(file: String) -> Result<SensorMap, Box<dyn Error>> {
    // create mut SensorMap
    let map: SensorMap;
    // serialize json config
    map = serde_json::from_reader(std::fs::File::open(file)?)?;
    // return
    Ok(map)
}

/// Struct which holds the device configs to log in to Azure IoT hub <config/sensor-data.json>.
#[derive(Debug, Deserialize, Clone)]
pub struct SensorData {
    pub sensortype: String,
    pub value: f32,
    pub timestamp: String,
}

impl SensorData {
    pub fn new(sensortype: String, value: f32, timestamp: String) -> Self {
        Self {
            sensortype: sensortype,
            value: value,
            timestamp: timestamp,
        }
    }
}

//-------------------------------------------------------------------------------------------------------
//                                             Global config
//-------------------------------------------------------------------------------------------------------
/// Struct which holds the device configs to log in to Azure IoT hub <config/config.toml>
#[derive(Debug, Deserialize)]
pub struct DeviceConfig {
    pub sub_ms01_socket: String,
    pub pub_ms02_socket: String,
    pub pub_ms01_topic: String,
    pub sub_ms02_topic: String,
    pub logging_path: String,
    pub logging_path_dev: String,
    pub sensor_data_path: String,
    pub sensor_data_path_dev: String,
    pub intervall_pub_ms01: u16

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

//-------------------------------------------------------------------------------------------------------
//                                             Unit testing
//-------------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sensor_map_path() {
        // test if the file path is not correct
        match create_sensor_map("config/sensor-data.json".to_string()) {
            Ok(i) => i,
            Err(e) => panic!(
                "Create SensorMap failed because of 1) wrong path 2) wrong json object: {:?}",
                e
            ),
        };
    }
}

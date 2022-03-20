// -----------------------------------------------------------------------------------------
//                                      Imports
// -----------------------------------------------------------------------------------------
use azure_iot_sdk::{
    DeviceKeyTokenSource, DirectMethodResponse, IoTHubClient, Message, MessageType,
};
use serde::Deserialize;
use std::{str, collections::HashMap};

// -----------------------------------------------------------------------------------------
//                                      Main
// -----------------------------------------------------------------------------------------
/// Hashmap as Cache for sensordata
pub type CachSensordata = HashMap<String, String>;

/// Struct which holds the information recieved and send by the side thread regarding the red sensor values via UART
#[derive(Debug, Deserialize, Clone)]
pub struct SensorDataTransmit {
    pub node: String,
    pub value: f32,
}

pub fn read_zmq(socket: &zmq::Socket) -> Option<String> {
    // get the event in ZMQ pipe
    match socket.get_events() {
        Ok(event) => {
            // Only work if Event is Polling
            if event == zmq::POLLIN {
                // get data
                let data = socket.recv_string(0).unwrap().unwrap();

                // only work with the data
                if data != "publish_01".to_owned() {
                    // Split the data string at ";" and push into buffer. Then remove the last item, since it is empty
                    let mut buffer: Vec<&str> = data.split(";").collect();
                    buffer.pop();

                    let data_str: String = buffer.concat();
                    return Some(data_str);
                };
            } else {
                return None;
            }
        }
        Err(e) => panic!("Failed to get ZMQ ms02 subscriber events with: {:?}", e),
    };
    return None;
}

pub fn write_zmq(
    socket: &zmq::Socket,
    config: &super::config::DeviceConfig,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get the msg data from msg.body
    let data = str::from_utf8(&msg.body)?;

    // check if it is only heartbeat. If yes return, if not publish via zmq
    if data == "hb" {
        // Debugging
        println!("Data recv from Azure IoT Hub: {:?}", data);
        return Ok(());
    }
    // If else publish the data via ZMQ
    else {
        // Debugging
        println!("Data recv from Azure IoT Hub and send via ZMQ: Data {:?} Topic: {:?}", data, &config.pub_ms01_topic);
        // send first topic and then complete data string
        socket.send(&config.pub_ms01_topic, zmq::SNDMORE)?;

        // send data
        socket.send(&data, 0)?;

        // Return ok
        return Ok(());
    }

    #[allow(unreachable_code)]
    // Return even if not reached
    Ok(())
}

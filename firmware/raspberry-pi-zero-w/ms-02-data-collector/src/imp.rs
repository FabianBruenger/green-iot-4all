use serde::Deserialize;
use std::{collections::HashMap, error::Error, str, str::FromStr};

// module file for implementing
pub mod config;
pub mod radiotransmit;
pub mod logs;

/// SensorPool is the central structs that holds all sensor data
#[derive(Debug, Deserialize, Clone)]
pub struct SensorPool {
    data: config::SensorMap,
}

impl SensorPool {
    // -----------------------------------------------------------

    /// creates new struct with already set up SensorMap.
    pub fn new(config: &config::DeviceConfig) -> Result<Self, Box<dyn std::error::Error>> {
        // select config depending on environment
        let mut path: &str;

        // Depending on Environment choose the path
        if cfg!(debug_assertions) {
            path = &config.sensor_data_path_dev;
        } else {
            path = &config.sensor_data_path;
        }


        // create SensorMap. If error, panic and abort process
        let map = config::create_sensor_map(path.to_string())?;

        // return
        Ok(SensorPool { data: map })
    }

    /// Update sensor data. When the subthread writes data the sensor data has to be updated
    pub fn update_data(&mut self, transmitdata: radiotransmit::SensorDataTransmit) {
        // cache for new structs
        let mut buffer: config::SensorMap = HashMap::new();

        // find the sensor with same node name
        for (key, val) in &self.data {
            if key == &transmitdata.node {
                // create a new struct and push to buffer
                let local_struct = config::SensorData::new(
                    val.sensortype.to_owned(),
                    transmitdata.value,
                    val.timestamp.to_owned(),
                );
                let local_name = key.to_owned();

                // insert in cache
                buffer.insert(local_name, local_struct);
            }
        }

        // update the cache
        for (key, val) in buffer {
            &self.data.insert(key, val);
        }
    }

    // -----------------------------------------------------------

    /// Transform all sensor data into a string for zmq publish. Syntax: <sensorname>:<value>;<sensorname2>:<value2>;...
    pub fn transform_data_to_string(&mut self) -> String {
        // get the data hash map
        let data = &self.data.to_owned();

        // create an empty string buffer
        let mut buffer: String = String::new();

        for (key, val) in data {
            // erase the ":" from nodes name, since ZMQ API expect only one ":" per data
            let mut buffer2: Vec<u8> = key.as_bytes().to_vec().to_owned();
            buffer2.remove(4);

            // insert the name
            buffer.push_str(&str::from_utf8(&buffer2).unwrap().to_owned());
            // value seperator :
            buffer.push_str(":");
            // insert value
            buffer.push_str(&val.value.to_string());
            // end seperator ;
            buffer.push_str(";");
        }

        buffer
    }

    /// With this function you can send the data directly one by one via ZMQ socket
    pub fn publish_data(&mut self, socket: &zmq::Socket) -> Result<(),Box<dyn Error>>{

        // get the data hash map
        let data = &self.data.to_owned();

        // create an empty string buffer
        let mut buffer: String = String::new();

        // iterate through the hash map and send the data via zmq
        for (key, val) in data {
            // erase the ":" from nodes name, since ZMQ API expect only one ":" per data
            let mut buffer2: Vec<u8> = key.as_bytes().to_vec().to_owned();
            buffer2.remove(4);

            // insert the name
            buffer.push_str(&str::from_utf8(&buffer2).unwrap().to_owned());
            // value seperator :
            buffer.push_str(":");
            // insert value
            buffer.push_str(&val.value.to_string());
            // end seperator ;
            buffer.push_str(";");

            // send first topic and then complete data string
            if let Err(e) = socket.send("publish_01", zmq::SNDMORE){
                log::error!("Could not send topic via ZMQ: {}", e);
            }

            if let Err(e) = socket.send(&buffer, 0){
                log::error!("Could not send msg via ZMQ: {}", e);
            }

            // debug
            println!("Published via zmq: {:?}", buffer);

            // clear buffer
            buffer.clear();

        }
        Ok(())
    }
}

pub fn subth_handle_ZMQ(socket: &zmq::Socket, sender: crossbeam_channel::Sender<radiotransmit::SensorDataTransmit>, config: &config::DeviceConfig) -> Result<(), Box<dyn Error>> {
    // get the event in ZMQ pipe
    match socket.get_events() {
        Ok(event) => {
            println!("Event from zmq sub: {:?}", event);
            // Only work if Event is Polling
            if event == zmq::POLLIN {
                // get data
                let data = socket.recv_string(0).unwrap().unwrap();

                // only work with the data
                if data != config.sub_ms02_topic {
                    // Split the data string at ";" and push into buffer. Then remove the last item, since it is empty
                    let mut buffer: Vec<&str> = data.split(":").collect();

                    // Create new msg SensorTransmit
                    let msg = radiotransmit::SensorDataTransmit {
                        node: buffer[0].to_string(),
                        value: f32::from_str(buffer[1]).unwrap()
                    };

                    // debug 
                    println!("Send to main thread: {:?}", msg);

                    // Send data to main thread
                    sender.send(msg).unwrap();

                    // return
                    return Ok(())
                };
            } else {
                return Ok(());
            }
        }
        Err(e) => panic!("Failed to get ZMQ ms02 subscriber events with: {:?}", e),
    };

    Ok(())
}

pub fn read_zmq(socket: &zmq::Socket, config: &config::DeviceConfig) -> Option<String> {


    // get the event in ZMQ pipe
    match socket.get_events() {
        Ok(event) => {
        
            // Only work if Event is Polling
            if event == zmq::POLLIN {
                // get data
                let data = socket.recv_string(0).unwrap().unwrap();

                // only work with the data
                if data != config.sub_ms02_topic {

                    // if the data is not the topic, then return the data
                    return Some(data);
                };
            } else {
                return None;
            }
        }
        Err(e) => panic!("Failed to get ZMQ ms02 subscriber events with: {:?}", e),
    };
    return None;
}
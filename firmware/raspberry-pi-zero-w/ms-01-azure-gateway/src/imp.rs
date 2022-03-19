// -----------------------------------------------------------------------------------------
//                                      Imports
// -----------------------------------------------------------------------------------------
use azure_iot_sdk::{
    DeviceKeyTokenSource, DirectMethodResponse, IoTHubClient, Message, MessageType,
};
use chrono::Utc;
use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::{error::Error, fs};
use zmq;

// -----------------------------------------------------------------------------------------
//                                      Exports
// -----------------------------------------------------------------------------------------
pub mod config;
pub mod sensordata;

// -----------------------------------------------------------------------------------------
//                                      Logging
// -----------------------------------------------------------------------------------------
pub fn set_logging(config: &config::DeviceConfig) -> Result<(), Box<dyn Error>> {
    // get current dat and time for logging file
    let now = Utc::now();
    let now_str = format!(
        "{}_{}",
        &config.logging_path.to_owned(),
        &now.format("%Y-%m-%d_%H:%M:%S").to_string()
    );

    // archiv the current log file. If it is not existing, then create new
    if let Err(_e) = fs::rename(&config.logging_path.to_owned(), now_str) {
        fs::File::create(&config.logging_path.to_owned()).unwrap();
    }

    // Set file appender
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[{d(%Y-%m-%d %H:%M:%S)}]{h({l})}: {m}\n",
        )))
        .build(&config.logging_path.to_owned())?;

    // Build the config
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

    // Final instance
    log4rs::init_config(config)?;

    // Debug
    log::info!("Logging set up initially");

    Ok(())
}

// -----------------------------------------------------------------------------------------
//                                      ZMQ
// -----------------------------------------------------------------------------------------
/// Create ZMQ subscriber socket
pub fn inst_zmq(config: &config::DeviceConfig, sel: &str) -> Result<zmq::Socket, Box<dyn Error>> {
    // create zmq instance
    let context_main = zmq::Context::new();

    // Match which kind of socket you want to create
    match sel {
        "SUB" => {
            // create subscriber
            let ms02_subscriber = context_main.socket(zmq::SUB)?;

            // TO DO handle error fro connecting
            ms02_subscriber.connect(&config.sub_ms02_socket)?;

            // set subscription to same topic as in ms02
            if let Err(e) = ms02_subscriber.set_subscribe(config.sub_ms02_topic.as_bytes()) {
                panic!("Set topic for ms02_subscriber failed with: {:?}", e)
            }

            log::info!(
                "Set up zmq subscriber socket, listening on {} with the topic {}!",
                &config.sub_ms02_socket,
                &config.sub_ms02_topic
            );

            // return the subscriber
            Ok(ms02_subscriber)
        }

        "PUB" => {
            // Create the publisher socket
            let publisher = match context_main.socket(zmq::PUB) {
                Ok(i) => i,
                Err(e) => panic!("Error while init the ZMQ socket: {}", e),
            };

            // Panic if not able to bind
            assert!(publisher.bind(&config.pub_ms01_socket).is_ok());

            log::info!(
                "Set up zmq publisher socket, publishing to: {}!",
                &config.pub_ms01_socket
            );

            // Return the publisher
            Ok(publisher)
        }

        _ => panic!("No valid select socket type"),
    }
}

// -----------------------------------------------------------------------------------------
//                                      Azure
// -----------------------------------------------------------------------------------------
/// Main Azure struct for operations on Azure IoT Hub
#[derive(Debug, Clone)]
pub struct AzureIOThub {
    pub client: IoTHubClient,
}

impl AzureIOThub {
    /// Create new instance
    pub async fn new(config: &config::DeviceConfig) -> Self {
        // genrate a token src
        let token_source = match DeviceKeyTokenSource::new(
            &config.hostname,
            &config.device_id,
            &config.shared_access_key,
        ) {
            Ok(data) => {
                log::info!("Set up token source");
                data
            }
            Err(e) => panic!(format!("Failed to create token source with : {:?}", e)),
        };

        // create a new Azure IoT hub client
        let client =
            match IoTHubClient::new(&config.hostname, config.device_id.to_owned(), token_source)
                .await
            {
                Ok(data) => {
                    log::info!("Set up IoT Hub client");
                    data
                }
                Err(e) => panic!(format!("Could not set up Azure IoZ Hub client: {:?}", e)),
            };

        // Return the client
        Self { client: client }
    }

    // /// Write data to Azure IoT Hub
    // pub async fn write(
    //     &mut self,
    //     msg: sensordata::SensorDataTransmit,
    // ) -> Result<(), Box<dyn std::error::Error>> {
    //     // Build Azure IoT hub msg
    //     let msg = Message::builder()
    //         .set_body(format!("{}:{}", msg.node, msg.value).as_bytes().to_vec())
    //         .set_message_id(format!("{}", 1))
    //         .build();

    //     // Send msg to Azure IoT Hub
    //     if let Err(e) = self.client.send_message(msg).await {
    //         log::error!("could not send msg to Azure IoT Hub: {:?}", e)
    //     };

    //     // Return Okey if everything worked fine
    //     Ok(())
    // }

    // /// Read from Azure IoT Hub
    // pub async fn read(&mut self) -> Result<String, Box<dyn std::error::Error>> {
    //     // read the reciever
    //     let mut receiver = self.client.get_receiver().await;

    //     println!("inside read fn");
    //     while let Some(msg) = receiver.recv().await {
    //         println!("got sthfrom Azure");
    //         // Handle MSGs
    //         match msg {
    //             MessageType::C2DMessage(msg) => {
    //                 // Handle the msg
    //                 // either it is the haertbeat or some data to write

    //                 // first parse msg.body
    //                 return Ok(std::str::from_utf8(&msg.body)?.to_string());
    //             }
    //             MessageType::ErrorReceive(err) => {
    //                 log::error!("Error during receive {:?}", err)
    //             }
    //             _ => return Ok("Done".to_string()),
    //         }
    //     }

    //     // Although never reached, just return Ok(())
    //     Ok("Done".to_string())
    // }
}

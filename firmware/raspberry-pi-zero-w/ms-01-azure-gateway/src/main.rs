// -----------------------------------------------------------------------------------------
//                                      Imports
// -----------------------------------------------------------------------------------------
use azure_iot_sdk::{
    Message, MessageType,
};
use tokio::{time};

extern crate log;
mod imp;

// -----------------------------------------------------------------------------------------
//                                      Main
// -----------------------------------------------------------------------------------------
#[tokio::main]
async fn main() {
    // -----------------------------------------------------------------------------------------
    //                                      Config
    // -----------------------------------------------------------------------------------------
    // create the config var
    let config = match imp::config::DeviceConfig::new() {
        Ok(data) => data,
        Err(e) => panic!("Could not read toml config: {:?}", e),
    };

    // -----------------------------------------------------------------------------------------
    //                                      Logging
    // -----------------------------------------------------------------------------------------

    if let Err(e) = imp::set_logging(&config) {
        panic!("Could not set up logging with: {:?}", e);
    }

    // -----------------------------------------------------------------------------------------
    //                                      Azure IoT Hub
    // -----------------------------------------------------------------------------------------

    // Creating Azure iot objects for reading and writing
    let mut client_tx = imp::AzureIOThub::new(&config).await;

    // Get reciever for reading Azure Hub
    let mut receiver = client_tx.client.get_receiver().await;

    // Count for dubbuging
    let mut count = 0u32;

    // Maybe not necessary
    let mut writer = client_tx.client.clone();

    // Time fro reading out of sensor pool
    let mut tck_zmq = time::interval(time::Duration::from_millis(config.intervall_zmq_rx as u64));
    // Create the count value for sending to Azure IoT Hub
    let mut tck_cnt = 0u8;

    // -----------------------------------------------------------------------------------------
    //                                      ZMQ config
    // -----------------------------------------------------------------------------------------

    let ms02_subscriber = match imp::inst_zmq(&config, "SUB") {
        Ok(sub) => sub,
        Err(e) => panic!(
            "Could not create instance of subscriber zmq socket with: {:?}",
            e
        ),
    };

    let ms01_publisher = match imp::inst_zmq(&config, "PUB") {
        Ok(sub) => sub,
        Err(e) => panic!(
            "Could not create instance of publisher zmq socket with: {:?}",
            e
        ),
    };

    // -----------------------------------------------------------------------------------------
    //                                      data cache
    // -----------------------------------------------------------------------------------------
    let mut data_cache = imp::sensordata::CachSensordata::new();

    // -----------------------------------------------------------------------------------------
    //                                      Run
    // -----------------------------------------------------------------------------------------

    // Read Azure IoT Hub async
    let receive_loop = async {
        loop {
            // debug
            println!("receive_loop");

            // Wait for data
            while let Some(msg) = receiver.recv().await {
                match msg {
                    // This is the real msg. Only handle this msg
                    MessageType::C2DMessage(msg) => {
                        if let Err(e) = imp::sensordata::write_zmq(&ms01_publisher, &config, msg) {
                            log::error!("Error while publishing data to zmq socket: {:?}", e)
                        }
                    }
                    // If we get an error, let the service panic since it is not clear what to do
                    MessageType::ErrorReceive(err) => panic!("Error during receive {:?}", err),
                    // All other msg are not treated yet
                    _ => println!("Received message {:?}", msg),
                }
            }
        }
    };

    // Read ZMQ socket
    let zmq_loop = async {
        loop {

            // Fire this loop every 200 ms. Might increase the data rate with increasing msgs
            tck_zmq.tick().await;

            // Read ZMQ socket
            if let Some(data) = imp::sensordata::read_zmq(&ms02_subscriber) {

                // Split data
                let buffer: Vec<&str> = data.split(":").collect();

                // Insert the sensordata
                &data_cache.insert(buffer[0].to_owned(), buffer[1].to_owned());
            }

            // Check if it is time to write to Azure IoT Hub
            if tck_cnt >= config.factor_azu_tx {

                // create string buffer for sending
                let mut data_send = String::new();

                // Write the data to azure IoT Hub every x seconds
                for (key, value) in &data_cache {
                    // buffer
                    // just insert the data
                    data_send.push_str(&format!("{}{}{}{}",key,':',value,';'));                    
                }

                let msg = Message::builder()
                    .set_body(format!("{}", data_send).as_bytes().to_vec())
                    .set_message_id(format!("{}", count))
                    .build();

                // Send the msg. Treat error!
                writer.send_message(msg).await?;

                // Check count
                if count >= 4_000_000 {
                    count = 0;
                } else {
                    count += 1;
                }

                // debug
                println!("Send to Azure IoT Hub: {:?}", &data_send);

                tck_cnt = 0;
            }
            // Otherwise dont to anything but counting up
            else{
                tck_cnt += 1;
            }
        }

        #[allow(unreachable_code)]
        Ok::<(), Box<dyn std::error::Error>>(())
    };


    let (_, _) = tokio::join!(receive_loop, zmq_loop);
}

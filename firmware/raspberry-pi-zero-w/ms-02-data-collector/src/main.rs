//! ms-02-data-collector: zmq server, read sensor data

#[allow(unused_variables)]
// -----------------------------------------------------------------------------------------
//                                      Imports
// -----------------------------------------------------------------------------------------
use crossbeam_channel::{select, tick, unbounded};
// use chrono::{Datelike, Timelike, Utc};
// use log::LevelFilter;
// use log4rs::{
//     append::file::FileAppender,
//     config::{Appender, Config, Root},
//     encode::pattern::PatternEncoder,
// };
// use std::fs;
use zmq;

// import modulesrust crossbeam channel structs
mod imp;


// -----------------------------------------------------------------------------------------
//                                      Main
// -----------------------------------------------------------------------------------------
fn main() {
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

    if let Err(e) = imp::logs::set_logging(&config) {
        panic!("Could not set up logging with: {:?}", e);
    }

    // -----------------------------------------------------------------------------------------
    //                                      Globals
    // -----------------------------------------------------------------------------------------

    // create SensorMap. If error, panic and abort process
    let mut sensordata = match imp::SensorPool::new(&config) {
        Ok(i) => {
            log::info!("created SensorPool main struct: {:?}", i);
            i},
        Err(e) => panic!("{}", e),
    };

    // debug
    println!("Init sensordata: {:#?}", sensordata);

    // -----------------------------------------------------------------------------------------
    //                                      ZMQ
    // -----------------------------------------------------------------------------------------
    let context = zmq::Context::new();

    let publisher = match context.socket(zmq::PUB) {
        Ok(_i) => _i,
        Err(e) => panic!("Error while init the ZMQ socket: {}", e),
    };

    // TO DO: Handle error
    assert!(publisher.bind(&config.pub_ms02_socket).is_ok());

    let context_2 = zmq::Context::new();

    // Subscriber to ms-01
    let subscriber = match context_2.socket(zmq::SUB) {
        Ok(_i) => _i,
        Err(e) => panic!("Error while init the ZMQ socket: {}", e),
    };

    // TO DO: Handle error
    assert!(subscriber.connect(&config.sub_ms01_socket).is_ok());

    // set subscription to same topic as in ms02
    if let Err(e) = subscriber.set_subscribe(config.sub_ms02_topic.as_bytes()) {
        panic!("Set topic for ms02_subscriber failed with: {:?}", e)
    }


    // -----------------------------------------------------------------------------------------
    //                                      Threads
    // -----------------------------------------------------------------------------------------
    // crossbeam channels for inter-threading communication

    // Channel for: Recieving UART sensor data for updating
    let (send_sensordata, recv_sensordata) = unbounded();
    // Channel for: Sending sensor data to UART thread
    let (send_sensordata_tx, recv_sensordata_tx) = unbounded();

    // clocking for publishing
    let tck_zmq = tick(std::time::Duration::from_millis(config.intervall_pub_ms01 as u64));

    // clocking for listening to sub
    let tck_sub = tick(std::time::Duration::from_millis(200));

    // fire subthread for reading the radio transmitted sensor data
    std::thread::spawn(move || {
        imp::radiotransmit::subth_handle_UART(send_sensordata, recv_sensordata_tx);
        log::info!("spawn subthread for listening to UART");
    });


    // -----------------------------------------------------------------------------------------
    //                                      Run
    // -----------------------------------------------------------------------------------------
    loop {
        select! {
            // recieveing the subth_handle_UART
            recv(recv_sensordata) -> msg => {
                sensordata.update_data(msg.unwrap());
                // println!("In main thread recieved & updated: {:#?}", sensordata);

            }

            //  PUBLISH DATA VIA ZMQ
            recv(tck_zmq) -> _ => {

                if let Err(e) = sensordata.publish_data(&publisher){
                    println!("Error while calling publish_data with: {:?}", e)
                }
            }

            // LISTEN / SUBSCRIBE TO ZMQ MS-01 SOCKET
            recv(tck_sub) -> _ => {

                if let Some(data) = imp::read_zmq(&subscriber , &config){
                    println!("Got data from ZMQ in main thread: {:?}", data);

                    // Send data via UART

                    if let Err(e) = send_sensordata_tx.send(data) {
                        log::error!("Could not send the data from main to side thread: {}", e)
                    }
                }

            }
        }
    }
}

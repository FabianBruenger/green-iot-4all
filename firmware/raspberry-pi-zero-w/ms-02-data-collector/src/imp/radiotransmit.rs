// -----------------------------------------------------------------------------------------
//                                      Imports
// -----------------------------------------------------------------------------------------
use crossbeam_channel::{unbounded, TryRecvError};
use rppal::uart::{Parity, Uart};
use serde::Deserialize;
use std::{error::Error, str, str::FromStr, time::Duration};

/// Struct which holds the information recieved and send by the side thread regarding the red sensor values via UART
#[derive(Debug, Deserialize)]
pub struct SensorDataTransmit {
    pub node: String,
    pub value: f32,
}

/// Holds all the required buffers and data for handle incoming UART request on the raspberry pi
struct UartData {
    data_buffer: Vec<u8>,
    uart_buffer: [u8; 1],
    uartobj: rppal::uart::Uart,
}

/// Methods for creating the object and handle the data
impl UartData {
    fn new(
        baud_rate: u32,
        parity: Parity,
        data_bits: u8,
        stop_bits: u8,
    ) -> Result<Self, Box<dyn Error>> {
        // create the vectors
        let buffer2: Vec<u8> = Vec::with_capacity(32);
        let buffer = [0u8; 1];

        // create the uart
        let mut uart = match Uart::new(baud_rate, parity, data_bits, stop_bits) {
            Ok(data) => data,
            Err(e) => {
                log::error!("UART object could not be created");
                return Err(Box::new(e));
            }
        };

        uart.set_read_mode(0, Duration::new(0, 0)).unwrap();

        Ok(Self {
            data_buffer: buffer2,
            uart_buffer: buffer,
            uartobj: uart,
        })
    }


    /// Read the UART data and sends it to main thread
    fn handle_uart(
        &mut self,
        sender: &crossbeam_channel::Sender<SensorDataTransmit>,
    ) -> Result<(), Box<dyn Error>> {
        // Fill the buffer variable with any incoming data.

        let payload = match self.uartobj.read(&mut self.uart_buffer) {
            Ok(recbytes) => recbytes,
            Err(e) => {
                log::error!("UART object could not be red");
                return Err(Box::new(e));
            }
        };

        if payload > 0 {
            // if the start byte recieved, check if last byte is the "E" or None, then it is a valid msg. If not, then throw error
            if self.uart_buffer[0] == 83 {
                match self.data_buffer.pop() {
                    Some(i) => {
                        if i == 69 {
                            self.data_buffer.push(69);
                            self.data_buffer.push(self.uart_buffer[0]);
                        } else {
                            self.data_buffer.clear();
                            log::warn!("Corrupted msg red from UART transmission");
                        }
                    }
                    None => {
                        self.data_buffer.push(self.uart_buffer[0]);
                    }
                }
            }
            // if end byte is recieved: print and clear the buffer
            else if self.uart_buffer[0] == 69 {
                // push the last character
                self.data_buffer.push(self.uart_buffer[0]);

                // transform buffer to node and value and create the struct
                let uart_data = SensorDataTransmit {
                    node: str::from_utf8(&self.data_buffer[1..8].to_vec())
                        .unwrap()
                        .to_owned(),
                    value: f32::from_str(
                        str::from_utf8(&self.data_buffer[9..&self.data_buffer.len() - 2].to_vec())
                            .unwrap(),
                    )
                    .unwrap(),
                };

                // debug
                // println!("debug: {:?}", uart_data);

                if let Err(e) = sender.send(uart_data) {
                    log::error!("Could not send the data from side to main thread: {}", e)
                }

                self.data_buffer.clear();

            // else just parse the msg object
            } else {
                self.data_buffer.push(self.uart_buffer[0]);
            }
        }

        Ok(())
    }


    /// Recieve UART data from main thread and sends it per UART
    fn write_uart(
        &mut self,
        reciever: &crossbeam_channel::Receiver<String>,
    ) -> Result<(), Box<dyn Error>> {

        // try to set write mode
        self.uartobj.set_write_mode(false)?;

        // If we can recieve from main thread just print the message
        if let Ok(data) = reciever.try_recv() {
            println!("From write UART fn: {:?}", data);

            // write UART with I/O data
            let result = self.uartobj.write(data.as_bytes())?;

            println!("result from sending uart: {:?}", result);
        }

        Ok(())
    }
}


/// define sensor data sub thread
pub fn subth_handle_UART(
    sender: crossbeam_channel::Sender<SensorDataTransmit>,
    reciever: crossbeam_channel::Receiver<String>,
) {
    // create the UART object
    let mut uart = match UartData::new(9600u32, Parity::None, 8u8, 1u8) {
        Ok(obj) => obj,
        Err(e) => panic!("Error while initializing UART: {}", e),
    };

    log::info!("created UART successfully");


    loop {
        // listen to UART
        if let Err(e) = uart.handle_uart(&sender) {
            log::error!("Handle Uart function throws error: {}", e);
        }


        // write to UART
        if let Err(e) = uart.write_uart(&reciever) {
            log::error!("Write Uart function throws error: {}", e);
        }

    }
}

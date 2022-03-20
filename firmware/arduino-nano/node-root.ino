// ----------------------------------------------------------------------------------------------//
//                                              Libs                                             //
// ----------------------------------------------------------------------------------------------//
#include <SPI.h>
#include <RF24.h>
#include <RF24Network.h>

// ----------------------------------------------------------------------------------------------//
//                                              Globals                                          //
// ----------------------------------------------------------------------------------------------//
// NRF24 init & NRF24 network init
RF24 radio(9, 10);
RF24Network network(radio);

// adress array
const uint16_t addresses[] = { 00, 01, 02, 03, 04 };

// intervall for transmitting in ms
const unsigned long txinterval = 3000;
// buffer for last msg in ms
unsigned long txlast;

// UART buffer
char UARTbuf[32];

// Transrecieving struct
struct msg {
  unsigned long node;
  unsigned long sensor;
  float data;
};
// ----------------------------------------------------------------------------------------------//
//                                              Helpers                                          //
// ----------------------------------------------------------------------------------------------//
String returnAdr(unsigned long input) {
  String out;
  if (input <= 9) {
    out = "000" + String(input);
  }
  else if (input >= 10 && input <= 99) {
    out = "00" + String(input);
  }
  else if (input >= 100 && input <= 999) {
    out = "0" + String(input);
  }
  else {
    out = String(input);
  }

  return out;
}



// ----------------------------------------------------------------------------------------------//
//                                              Init                                             //
// ----------------------------------------------------------------------------------------------//
void setup(void) {

  

  // UART init
  Serial.begin(9600);

  // Wait for the serial init
  if (!Serial) {
    // some boards need this because of native USB capability
  }
  

  // Debugging
  // Serial.println("Initialize Node00");

  // Start SPI
  SPI.begin();

  // Init radio
  if (!radio.begin()) {
    // Serial.println("Radio hardware not responding!");
    while (1) {
      // hold in infinite loop
    }
  }

  // Start networking
  network.begin(77, addresses[0]);

  // test diff speed
  radio.setDataRate(RF24_2MBPS);
}


// ----------------------------------------------------------------------------------------------//
//                                              Run                                              //
// ----------------------------------------------------------------------------------------------//
void loop(void) {

  // Network update first command always
  network.update();

  // ------------------------------------- Reading network -------------------------------------//
  while (network.available()) {

    // create header
    RF24NetworkHeader headerrx;
    // create object
    msg datarx;

    // read data
    network.read(headerrx, &datarx, sizeof(datarx));

    // transform node address
    String address = returnAdr(datarx.node);

    // Build msg for gateway
    String msggw = "S" + address + ":0" + String(datarx.sensor) + "{" + String(datarx.data) + "}" + "E";

    Serial.print(msggw);
  }



  // -------------------------------- Handle UART rx & Transmit network ------------------------//
  if (Serial.available()) {

    // read data as String. TO DO test from raspi
    String data = Serial.readString();

    // Testing for gateway
    Serial.print(data);

    // cast data
    data.toCharArray(UARTbuf, 32);

    // build object for sending

    // address & header
    char adrRaw[4] = {0};
    char adrSen[2] = {0};
    char adrDat[1] = {0};
    int adrInt;
    int senInt;
    int datInt;
    uint16_t headertx;

    // --------------- Get Raw data

    // Filter address
    for (int i = 1; i <= 4; i++) {

      // save int address for struct msg
      adrRaw[i - 1] = UARTbuf[i];
    }

    // filter sensor address
    adrSen[0] = UARTbuf[6];
    adrSen[1] = UARTbuf[7];

    // Filter data (only 1 / 0)
    adrDat[0] = UARTbuf[9];

    // check and set header with compare
    for (int i = 0; i <= 4; i++) {

      // check on 1
      if (UARTbuf[i] == '1') {

        // set header to next node:
        headertx = 01;
        break;
      }
      // check on 2
      else if (UARTbuf[i] == '2') {

        // set header to next node:
        headertx = 02;
        break;
      }
      // check on 3
      else if (UARTbuf[i] == '3') {
        // set header to next node:
        headertx = 03;
        break;
      }
      // check on 4
      else if (UARTbuf[i] == '4') {
        // set header to next node:
        headertx = 04;
        break;
      }
    }

    // cast for object for sending
    adrInt = atoi(adrRaw);
    senInt = atoi(adrSen);
    datInt = atoi(adrDat);

    // Build msg object
    msg uartTX = { (unsigned long) adrInt , (unsigned long) senInt, (float) datInt};

    // Serial.print("Tx: "); Serial.print(uartTX.node); Serial.print(" "); Serial.print(uartTX.sensor); Serial.print(" "); Serial.println(uartTX.data);

    // set header of target node
    RF24NetworkHeader header(headertx);

    bool ok = network.write(header, &uartTX, sizeof(uartTX));
  }


}

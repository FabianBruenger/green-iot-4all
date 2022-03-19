// ----------------------------------------------------------------------------------------------//
//                                              Libs                                             //
// ----------------------------------------------------------------------------------------------//
#include <SPI.h>
#include <RF24.h>
#include <RF24Network.h>
#include <Adafruit_AHTX0.h>
#include <Wire.h>
#include <BH1750.h>
#include <Adafruit_CCS811.h>
#include <printf.h>

// ----------------------------------------------------------------------------------------------//
//                                              Globals                                          //
// ----------------------------------------------------------------------------------------------//
// NRF24 init & NRF24 network init
RF24 radio(9, 10);
RF24Network network(radio);

// Sensors
Adafruit_AHTX0 aht;
BH1750 lightMeter;
Adafruit_CCS811 ccs;

// adress array
const uint16_t addresses[] = {00, 01, 011, 012, 013};
// sensors nummer
const uint16_t sensorsnr[] = {1, 2, 3, 4, 5};

// intervall for transmitting in ms
const unsigned long txinterval = 50;
const unsigned long srinterval = 2000;

// buffer for last msg in ms
unsigned long txlast;

// Iterator for sending sensor values
unsigned short iterator;
bool ok;

// Transrecieving struct
struct msg
{
  unsigned long node;
  unsigned long sensor;
  float data;
};

// Sensor objects
msg TempData = {addresses[1], sensorsnr[0], 0};
msg HumdData = {addresses[1], sensorsnr[1], 0};
msg LighData = {addresses[1], sensorsnr[2], 0};
msg COtwData = {addresses[1], sensorsnr[3], 0};
msg TVOCData = {addresses[1], sensorsnr[4], 0};

// ----------------------------------------------------------------------------------------------//
//                                              Init                                             //
// ----------------------------------------------------------------------------------------------//
void setup(void)
{

  // Set iterator initially
  iterator = 0;

  // UART init
  Serial.begin(9600);
  printf_begin();

  // Wait for the serial init
  if (!Serial)
  {
    // some boards need this because of native USB capability
  }

  // Debugging
  Serial.print("Initialize Node: ");
  Serial.println(addresses[1]);

  // Sensors init
  if (!aht.begin())
  {

    // Debugging
    Serial.println("Could not find AHT? Check wiring");
    while (1)
      delay(10);
  }

  // Debugging
  Serial.println("AHT10 or AHT20 found");

  // Light sensor init
  Wire.begin();
  lightMeter.begin();

  // Debugging
  Serial.println("light sensor found");

  // CCS811 Gas sensor init
  if (!ccs.begin())
  {
    // debugging
    Serial.println("Failed to start ccs sensor! Please check your wiring.");
    while (1)
      ;
  }

  // Start SPI
  SPI.begin();

  // Init radio
  if (!radio.begin())
  {
    Serial.println(F("Radio hardware not responding!"));
    while (1)
    {
      // hold in infinite loop
    }
  }

  // Start networking
  network.begin(77, addresses[1]);

  // test diff speed
  radio.setDataRate(RF24_2MBPS);

  // debug settings
  radio.printDetails();
}

// ----------------------------------------------------------------------------------------------//
//                                              Run                                              //
// ----------------------------------------------------------------------------------------------//
void loop()
{

  // Network update first command always
  network.update();

  // ------------------------------------- Reading network  -------------------------------------//
  while (network.available())
  {

    // create header
    RF24NetworkHeader headerrx;
    // create object
    msg datarx;

    // read data
    network.read(headerrx, &datarx, sizeof(datarx));

    // Build msg for gateway
    String msggw = "S" + String(datarx.node) + ":" + String(datarx.sensor) + "{" + String(datarx.data) + "}" + "E";

    Serial.println(msggw);
  }

  // ------------------------------------- Sending network  -------------------------------------//

  // set header of target node
  RF24NetworkHeader header(addresses[0]);

  // use switch Case to send data organized
  switch (iterator)
  {
  // Send Temp data
  case 0:
    // Try to send
    ok = network.write(header, &TempData, sizeof(TempData));
    // If data got send, increment "iterator" and send next data
    if (ok)
    {
      iterator++;
      // Debug
      // Serial.println("Send data Temp");
      break;
    }
    else
    {
      // Serial.println("failed Temp");
      break;
    }
    break;
  // Send Hum data
  case 1:
    // Try to send
    ok = network.write(header, &HumdData, sizeof(HumdData));
    // If data got send, increment "iterator" and send next data
    if (ok)
    {
      iterator++;
      // Debug
      // Serial.println("Send data Hum");
      break;
    }
    else
    {
      // Serial.println("failed Hum");
      break;
    }
    break;
  // Send Light data
  case 2:
    // Try to send
    ok = network.write(header, &LighData, sizeof(LighData));
    // If data got send, increment "iterator" and send next data
    if (ok)
    {
      iterator++;
      // Debug
      // Serial.println("Send data Light");
      break;
    }
    else
    {
      // Serial.println("failed Light");
      break;
    }
    break;
  // Send CO2 data
  case 3:
    // Try to send
    ok = network.write(header, &COtwData, sizeof(COtwData));
    // If data got send, increment "iterator" and send next data
    if (ok)
    {
      iterator++;
      // Debug
      // Serial.println("Send data CO2");
      break;
    }
    else
    {
      // Serial.println("failed CO2");
      break;
    }
    break;
  // Send TVOC data
  case 4:
    // Try to send
    ok = network.write(header, &TVOCData, sizeof(TVOCData));
    // If data got send, increment "iterator" and send next data
    if (ok)
    {
      iterator=0;
      // Debug
      // Serial.println("Send data TVOC");
      break;
    }
    else
    {
      // Serial.println("failed TVOC");
      break;
    }
    break;
  default:
    // Reset iterator
    iterator = 0;
    break;
  }

  // ------------------------------------- Reading sensors  -------------------------------------//

  // get current time
  unsigned long now = millis();

  // read sensor data
  if (now - txlast >= srinterval)
  {

    // update timestamp
    txlast = now;

    // read data from sensors
    // AHT20
    // create instance
    sensors_event_t humidity, temp;
    // reading
    aht.getEvent(&humidity, &temp);

    // Update sensor data
    TempData.data = temp.temperature;
    HumdData.data = humidity.relative_humidity;

    // LightMeter
    LighData.data = lightMeter.readLightLevel();

    // ccs gas sensor
    if (ccs.available())
    {
      if (!ccs.readData())
      {

        // get CO2 data
        COtwData.data = ccs.geteCO2();

        // get TVOC data
        TVOCData.data = ccs.getTVOC();
      }
    }

    // debugging
    Serial.print("Tx-Temp: "); Serial.print(TempData.node); Serial.print(" "); Serial.print(TempData.sensor); Serial.print(" "); Serial.println(TempData.data);
    Serial.print("Tx-Humd: "); Serial.print(HumdData.node); Serial.print(" "); Serial.print(HumdData.sensor); Serial.print(" "); Serial.println(HumdData.data);
    Serial.print("Tx-Ligh: "); Serial.print(LighData.node); Serial.print(" "); Serial.print(LighData.sensor); Serial.print(" "); Serial.println(LighData.data);
    Serial.print("Tx-CO2 : "); Serial.print(COtwData.node); Serial.print(" "); Serial.print(COtwData.sensor); Serial.print(" "); Serial.println(COtwData.data);
    Serial.print("Tx-TVOC: "); Serial.print(TVOCData.node); Serial.print(" "); Serial.print(TVOCData.sensor); Serial.print(" "); Serial.println(TVOCData.data);
  }
}

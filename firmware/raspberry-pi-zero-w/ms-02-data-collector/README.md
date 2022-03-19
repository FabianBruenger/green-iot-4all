# Mikroservice 02: ms-02-data-collector

> This microservice is provides reads the sensor data from the radio transmitter and sends the data via zmq

## Testing
For testing the functionality is to run a counting loop and sending the data via zmq.

# Sensor config and development

## Sensor indecators
- LI = light sensor analog [lux]
- TP = temperature sensor digital DHT11 (firmware = *100, data collector = /100) [°C]
- HU = humidity sensor digital DHT11 (firmware = *100, data collector = /100) [rel]
- IO = actuator sensor shield. only writes 0 or 1 [bool]
- G1 = Gas sensor CO2 [ppm]
- G2 = Gas sensor TVOC [ppb]
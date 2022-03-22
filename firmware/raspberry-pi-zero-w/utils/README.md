# Pi set up as data collector (via radio transmit) and gateway to Azure IoT Hub

## Pi Installation for local development
1. Configure the SD card with Raspberry image app. Install the std image. Congirue pi in headless mode
2. Create an empty ssh for ssh enablement while boot
3. Create the wsp file with wlan configs https://www.raspberrypi.org/documentation/configuration/wireless/headless.md
4. Start the pi, check connection in local wlan and connect to pi via ssh: ssh pi@<ip-address> (raspberry)
5. For installing the NRF24 lib follow this:
6. Increase the [swap size](https://wpitchoune.net/tricks/raspberry_pi3_increase_swap_size.html)
7. Set up pi as hotspot [Hotspot](https://www.raspberryconnect.com/projects/65-raspberrypi-hotspot-accesspoints/158-raspberry-pi-auto-wifi-hotspot-switch-direct-connection)
8. [Save image ](https://howchoo.com/pi/create-a-backup-image-of-your-raspberry-pi-sd-card-in-mac-osx)

## Software development

First you have to install all dependencies:

- sudo apt-get update && sudo apt-get upgrade
- install cargo / rust
- install cargo deb: cargo install cargo-deb
- apt-get install libzmq3-dev (zmq)

### Cross compile for pi zero
- cargo build --target arm-unknown-linux-gnueabi
- docker start <container-name>
- docker exec -ti nifty_shirley sh -c "rm -r /opt/ms-02-data-collector" 
- docker cp ms-02-data-collector nifty_shirley:/opt
- docker exec -ti nifty_shirley sh -c "cd /opt/ms-02-data-collector && cargo build --target arm-unknown-linux-gnueabi"
- docker exec -ti nifty_shirley sh -c "cd /opt/ms-02-data-collector && cargo deb --target=arm-unknown-linux-gnueabi"

- scp /opt/ms-02-data-collector/target/arm-unknown-linux-gnueabi/debug/ms-02-data-collector pi@192.168.0.67:/home/pi
- docker exec -ti nifty_shirley sh -c "scp /opt/ms-02-data-collector/target/arm-unknown-linux-gnueabi/debug/ms-02-data-collector pi@192.168.0.67:/home/pi"

- docker exec -ti nifty_shirley sh -c "scp /opt/ms-02-data-collector/target/arm-unknown-linux-gnueabi/debian/ms-02-data-collector_0.1.0_armel.deb pi@192.168.0.67:/home/pi"

## C for Pi zero
- `cc server.c -o server -lzmq`to compile with zmq
- Copy files to cocker image for testing zmq: `docker cp server.c nifty_shirley:/opt/testing_zmq`
- make without Makefile: `g++ -Ofast -mfpu=vfp -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -std=c++0x -Wall -I../ ms2-nrf24-driver.c -lrf24-bcm -lrf24network -lzmq -o nrf24`

## Node-red
1. Changin favicon: https://gist.github.com/mohnen/6923d5eb2e4547bb7e5bd90546d2ee80

# Original source.list
deb http://raspbian.raspberrypi.org/raspbian/ buster main contrib non-free rpi
# Uncomment line below then 'apt-get update' to enable 'apt-get source'
#deb-src http://raspbian.raspberrypi.org/raspbian/ buster main contrib non-free rpi
deb https://growiot:CQbCYur6du8JuB@growiotartifactory.jfrog.io/artifactory/growiotdebian-debian-local trusty private

# For packagin
- add the solve URL for the artifactory (local)
- add the auth file for log in
- add dpkg architectue (armel)
- dpkg --print-architecture


# Set Up Azure ioT Hub
1. Create Hub
2. Get connection string with: az iot hub connection-string show
3. Register device
4. In the ms-01 config insert: device name, Azure IoT Hub name, device.primary-key
5. In Node-Red: Paste the IoT connection string which got by the cli command
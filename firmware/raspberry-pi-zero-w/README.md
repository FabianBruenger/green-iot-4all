


## Node-red
1. Changin favicon: https://gist.github.com/mohnen/6923d5eb2e4547bb7e5bd90546d2ee80


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

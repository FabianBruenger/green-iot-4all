FROM rust:1.59.0-slim

# Prevent any error messages about there not being a terminal
ENV DEBIAN_FRONTEND noninteractive
# Allow pkg-config to find 
ENV PKG_CONFIG_PATH=/usr/lib/arm-linux-gnueabihf/pkgconfig
# RPI tools dir
ENV RPI_TOOLS=/rpi_tools

# Enable the armhf arch
RUN dpkg --add-architecture armhf
RUN dpkg --add-architecture armel

RUN apt-get update -qq && \
    # Install the necessary packages
    # libudev-dev will also bring in the arm libc6 and gcc packages
    apt-get install -qq --no-install-recommends \
        git \
        pkg-config \
        libudev-dev:armhf \
        libudev-dev:arm \
        libssl-dev \
        wget \
        libzmq3-dev \
        libczmq-dev && \
    # Add the RPI toolchain
    git -C "/" clone -q --depth=1 https://github.com/raspberrypi/tools.git "${RPI_TOOLS}" && \
    # Remove most of the repo we just downloaded as we only need a small amount
    rm -fr "${RPI_TOOLS}/.git" \
           "${RPI_TOOLS}/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian" \
           "${RPI_TOOLS}/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64" && \
    # Then get rid of git as we only needed it to fetch the rpi tools
    apt-get purge -qq git && \
    # Purge anything that has become useless
    apt-get autoremove -qq --purge && \
    # And finally do cleanup
    apt-get clean -qq && rm -fr /var/lib/apt/* /var/cache/apt/*

# Enable arm v6 in Rust
RUN rustup target add arm-unknown-linux-gnueabihf
RUN rustup target add arm-unknown-linux-gnueabi

# insall cMake for c-libs (zmq)
RUN wget https://github.com/Kitware/CMake/releases/download/v3.20.0/cmake-3.20.0.tar.gz
RUN tar -zxvf cmake-3.20.0.tar.gz
RUN cd cmake-3.20.0 && ./bootstrap
RUN cd cmake-3.20.0 && make
RUN cd cmake-3.20.0 && make install

# CMD cargo build --release --target=arm-unknown-linux-gnueabihf
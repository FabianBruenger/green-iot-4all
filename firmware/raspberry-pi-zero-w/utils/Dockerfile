FROM ubuntu:20.04

# --------------------------- Init --------------------------- #

# Update default packages
RUN apt-get -qq update


# Get Ubuntu packages
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y -q \
    build-essential \
    curl \
    git \
    nano \ 
    wget \ 
    apt-utils \
    gcc-arm-linux-gnueabi \ 
    gcc-arm-linux-gnueabihf \
    g++-arm-linux-gnueabi \
    make \ 
    git-core \ 
    ncurses-dev \
    libc6-armel-cross \
    libc6-dev-armel-cross \
    binutils-arm-linux-gnueabi \
    libncurses5-dev \
    libc6-armhf-cross \
    libc6-dev-armhf-cross \
    libssl-dev \
    pkg-config \
    libzmq3-dev \
    libczmq-dev \
    bash-completion

# --------------------------- Rust --------------------------- #

# Get Rust; NOTE: using sh for better compatibility with other base images
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Check cargo is visible
RUN cargo --help

# TODO: line 49 and 54 seems not beeing stored.. run these commands in your pipeline again.
# install pi zero w toolchain
RUN rustup target add arm-unknown-linux-gnueabihf
RUN rustup target add arm-unknown-linux-gnueabi 

# Install cargo deb for building
RUN cargo install cargo-deb

RUN rustup default stable
RUN which rustc
RUN rustc --version

# Add a cargo config for reference the linker
COPY firmware/raspberry-pi-zero-w/utils/config /root/.cargo 

# download pi toolchains
RUN git clone https://github.com/raspberrypi/tools.git /opt/pi-toolchains

# insall cMake for c-libs (zmq)
RUN wget https://github.com/Kitware/CMake/releases/download/v3.20.0/cmake-3.20.0.tar.gz
RUN tar -zxvf cmake-3.20.0.tar.gz
RUN cd cmake-3.20.0 && ./bootstrap
RUN cd cmake-3.20.0 && make
RUN cd cmake-3.20.0 && make install

# Add armel for pi zero build with no hf
RUN dpkg --add-architecture armel
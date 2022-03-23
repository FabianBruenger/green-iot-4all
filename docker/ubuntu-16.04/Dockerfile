FROM ubuntu:16.04

# Update default packages
RUN apt-get -qq update

# Unable interactive install
RUN DEBIAN_FRONTEND=noninteractive

# Get Ubuntu packages
# libssl-dev = for openssl-sys crate
# build-essential = cmake
RUN apt-get install -y -q \
    build-essential \
    manpages-dev \
    curl \
    git \
    nano \ 
    wget \ 
    gcc-arm-linux-gnueabi \ 
    g++-arm-linux-gnueabi \
    libssl-dev \
    openssh-client \
    openssh-server

# insall cMake for c-libs (zmq)
RUN wget https://github.com/Kitware/CMake/releases/download/v3.20.0/cmake-3.20.0.tar.gz
RUN tar -zxvf cmake-3.20.0.tar.gz
RUN cd cmake-3.20.0 && ./bootstrap
RUN cd cmake-3.20.0 && make
RUN cd cmake-3.20.0 && make install

# Get Rust; NOTE: using sh for better compatibility with other base images
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# install pi zero w toolchain armv6 no hard float
RUN rustup target add arm-unknown-linux-gnueabi 

# Install cargo deb for building
RUN cargo install cargo-deb

# Add a cargo config for reference the linker
COPY docker/ubuntu-16.04/config /root/.cargo 

# download pi toolchains
RUN git clone https://github.com/raspberrypi/tools.git /opt/pi-toolchains

# Add armel for pi zero build with no hf
RUN dpkg --add-architecture armel
FROM ubuntu:14.04.3
RUN sudo apt-get -qqy install wget curl
RUN wget https://static.rust-lang.org/rustup.sh
RUN chmod +x rustup.sh
RUN ./rustup.sh --yes
RUN sudo apt-get -qqy install g++-arm-linux-gnueabihf
RUN mkdir ~/.cargo
ADD cargo_config ~/.cargo/cargo_config
VOLUME /source

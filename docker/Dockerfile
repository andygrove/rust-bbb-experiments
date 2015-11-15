FROM ubuntu:14.04
RUN apt-get update
RUN apt-get -qqy install gcc wget curl sudo
RUN wget https://static.rust-lang.org/rustup.sh
RUN chmod +x rustup.sh
RUN ./rustup.sh --yes
RUN apt-get -qqy install g++-arm-linux-gnueabihf
ADD rustlib-1.4.0-stable-2015-10-28-8ab8581-arm-unknown-linux-gnueabihf-75938b7c6f49a8e0a429f25b05d3342b52ade02a.tar.gz /usr/local
VOLUME /source

# rust-bbb-experiments

Experimenting with Rust on the Beaglebone Black.

NOTE: I don't have anything working yet. I'm learning from https://github.com/posborne/rust-sysfs-gpio and plan to use this is a base for developing some sample code for various aspects of robotics, such as interfacing with ultrasonic sensors, motor controllers, and SPI devices.

There are two main approaches to compiling Rust code for Beaglebone. One approach is to install Rust and Cargo on the device itself. The other approach is to cross compile from another computer.

I am trying both approaches. This project contains a Dockerfile to create a docker image set up for cross compiling to the Beaglebone. Although this appears to work, the resulting executables target a different version of GLIBC that the version installed on my Beaglebone. I am in the process of upgrading my Beaglebone to a more recent Debian build to try and address that.

## Preparing the Beaglebone Black

I am using the 2015-03-01 official Debian 7.8 image. To upgrade to this, follow the excellent instructions on Derek Molloy's site (I also recommend buying his book).

http://derekmolloy.ie/write-a-new-image-to-the-beaglebone-black/#Flashing_the_BBB_with_the_SD_Card_Image

## Building the Docker image for cross-compiling

- Go to https://sothr.com/RustBuild/armv7/rustlib/stable/latest and download the file to the `docker` directory. 
- Update `Dockerfile` with the correct filename (at time of writing, the filename is `rustlib-1.4.0-stable-2015-10-28-8ab8581-arm-unknown-linux-gnueabihf-75938b7c6f49a8e0a429f25b05d3342b52ade02a.tar.gz`
- Run `./build.sh`

## Compiling projects

NOTE: need to manually deploy ~/.cargo/config with these contents

```
[target.arm-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

```
docker run -t -i -v `pwd`:/source andygrove/rust_bbb
cd /source
cargo build --target=arm-linux-gnueabihf-gcc

```

## Deploy to Beaglebone

```
scp target/arm-linux-gnueabihf/blink-led root@192.168.7.2:
```


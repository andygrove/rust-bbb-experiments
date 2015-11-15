# rust-bbb-experiments

Experimenting with Rust on the Beaglebone Black.

NOTE: I don't have anything working yet. I'm learning from https://github.com/posborne/rust-sysfs-gpio and plan to use this is a base for developing some sample code for various aspects of robotics, such as interfacing with ultrasonic sensors, motor controllers, and SPI devices.

## Preparing the Beaglebone Black

```
apt-get update
apt-get install gcc
```

## Building the Docker image

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


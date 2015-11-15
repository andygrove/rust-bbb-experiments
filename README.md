# rust-bbb-experiments

Experimenting with Rust on the Beaglebone Black.

# Goals

I intend to add code samples to this repo to demonstrate using Rust to make use of the Beaglebone's GPIO (General Purpose Input Ouptut) pins for tasks such as interacting with sensors and actuators and interfacing with devices that support the SPI (Serial Peripheral Interface) protocol.

My main motivation is learning how to use Rust for some simple embedded projects.

# Getting Started

## Preparing the Beaglebone Black

The first step is to upgrade to the 2015-03-01 official Debian 7.8 image.

For detailed instructions, see Derek Molloy's [instructions](http://derekmolloy.ie/write-a-new-image-to-the-beaglebone-black/). Here is the brief version for ubuntu.

Burning the flasher image to an SD card from ubuntu.

```
$ sudo dd if=BBB-eMMC-flasher-debian-7.8-lxde-4gb-armhf-2015-03-01-4gb.img of=/dev/sdd

7577600+0 records in
7577600+0 records out
3879731200 bytes (3.9 GB) copied, 1078.29 s, 3.6 MB/s
```

Disconnect everything from the Beaglebone Black (really, everything) and then insert the SD card and hold down the boot button (the microswitch near the SD card slot) while inserting the 5V power supply (powering from USB is **not** sufficient) and wait for the LEDs to start flashing, then release the button. Once the image has been installled, the LEDs will be solid (takes 10 minutes or so).

# Building Rust projects for Beaglebone Black

There are two main approaches to compiling Rust code for Beaglebone. One approach is to install Rust and Cargo on the device itself. The other approach is to cross compile from another computer.

## Install Rust and Cargo from unofficial pre-built binaries

Grab the latest stable binaries from https://github.com/warricksothr/RustBuild (under the heading `ARMv6-armhf`) and untar them in `/usr` on the Beaglebone.

## Cross-compiling from another computer

This project contains a Dockerfile to create a docker image set up for cross compiling to the Beaglebone. Although this appears to work, the resulting executables target a different version of GLIBC that the version installed on my Beaglebone and I am still investigating how to resolve that.

To build the docker image for compiling projects:

- Go to https://sothr.com/RustBuild/armv7/rustlib/stable/latest and download the file to the `docker` directory.
- Update `Dockerfile` with the correct filename (at time of writing, the filename is `rustlib-1.4.0-stable-2015-10-28-8ab8581-arm-unknown-linux-gnueabihf-75938b7c6f49a8e0a429f25b05d3342b52ade02a.tar.gz`
- Run `./build.sh`

## Compiling projects

Create `~/.cargo/config` with these contents (add add these lines to your existing config).

```
[target.arm-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

Then to compile your project, navigate to the source directory on the host system and run:

```
docker run -t -i -v `pwd`:/source andygrove/rust_bbb
cd /source
cargo build --target arm-unknown-linux-gnueabihf
```

Then deploy the binary to the Beaglebone using scp:

```
scp target/arm-linux-gnueabihf/blink-led root@192.168.7.2:
```

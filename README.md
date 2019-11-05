# rpi-mmal-rs

This project uses [rust-bindgen](https://github.com/rust-lang/rust-bindgen)
to make a simple Rust binding for the `MMAL` library of Raspberry Pi.

MMAL (Multi-Media Abstraction Layer) is a framework which is used to provide a
host-side, simple and relatively low-level interface to multimedia components
running on VideoCore. The source code of RPI MMAL could be reviewed in project
[userland](https://github.com/raspberrypi/userland/tree/master/interface/mmal).

This project is low-level and simple binding for using RPI camera. You could use
[rpi-video-rs](https://github.com/Pragmatic-Elixir-Meetup/rpi-video-rs) simply
for H264 video record.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rpi-mmal-rs = "0.0.3"
```

## Development

Since this project sets the default target to `arm-unknown-linux-gnueabihf`.
Developers should compile the code on a real RPI device or cross-compiling
environment. We offer a Dockerfile (in folder `tools/docker/`) which is a
separate cross-compiling environment.

Users or contributors should install the standard Rust development environment.
And then adds Rust targets as below.

```
rustup target add arm-unknown-linux-gnueabihf
rustup target add armv7-unknown-linux-gnueabihf
```

## TODO

1. Integrates with a CI for testing successful compilation.

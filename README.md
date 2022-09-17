# smart-clock

Software for a Raspberry-Pi powered smart clock

## Building

Built with Ubuntu 20.04 LTS. Building on other platforms will require modification to `.cargo/config.toml`

- Install the Rust ARM7 toolchain (supports Pi 2/3/4)
  - `rustup target add armv7-unknown-linux-gnueabihf`
- Install [ARM GNU toolchain](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads)
  - Should be labeled (`arm-none-linux-gnueabihf`)
  - May be available in your package manager (eg. `sudo apt-get install g++-10-multilib-arm-linux-gnueabihf`)
  - Should be a C++ compiler (to build dependencies)
- Build
  - `cargo build`

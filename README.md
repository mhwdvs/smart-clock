# smart-clock

Software for a Raspberry-Pi powered smart clock

## Raspberry Pi Configuration

### Operating System

This software should be compatible with any ARMV7 OS image. I would recommend DietPi, since it's very lightweight.

- Flash DietPi to an SD card
- Unplug and plug back in to remount the SD card
- Edit `dietpi/dietpi.txt` to suit your needs, and then copy to the root of SD card
- Edit `dietpi-wifi.txt` at the root of the SD card
- Disconnect SD card from PC and install in the Raspberry Pi
- Power on Raspberry Pi, wait ~5 minutes for installation to complete

```
Default static IP: 192.168.1.77
Default username: root
Default credentials: ssh-key
```

#### Why not bare-metal?

It's quite possible to compile a Rust application into a micro-controllers bare-metal kernel. The main issue is that when targeting bare metal you lose access to the Rust standard library that . Additionally, there's some hardware that my software ideally _doesn't_ want to be responsible for such as the WiFi adapter

### Ansible

To deploy the software we will utilise Ansible to make the process streamlined and repeatable.

- Install Ansible on your host machine
- Add ssh connection details to /etc/ansible/hosts, under a new `smart-clock`
- Execute Ansible playbook: `ansible-playbook -i smart-clock ansible/main.yml`

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

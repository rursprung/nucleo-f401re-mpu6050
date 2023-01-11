# MPU6050 (IMU) Example
[![CI](https://github.com/rursprung/nucleo-f401re-mpu6050/actions/workflows/CI.yml/badge.svg)](https://github.com/rursprung/nucleo-f401re-mpu6050/actions/workflows/CI.yml)

This example showcases how the [mpu6050](https://crates.io/crates/mpu6050) crate for the IMU of the same name can be used on an STM32F4 chip.

The example logs messages using [`defmt`](https://defmt.ferrous-systems.com/).

The example has been tested on a [ST Nucleo-F401RE](https://www.st.com/en/evaluation-tools/nucleo-f401re.html) development
board but should work on any STM32F4xx family microcontroller as long as the IMU is connected via I2C1 on pins `PB8` (SCL) and `PB9` (SDA)
or the code is adapted accordingly.

## Prerequisites
1. [Install Rust](https://www.rust-lang.org/tools/install)
1. Optional: ensure that the rust toolchain is up-to-date: `rustup update`
1. Install [`probe-run`](https://crates.io/crates/probe-run): `cargo install probe-run`
1. Install [`flip-link`](https://crates.io/crates/flip-link): `cargo install flip-link`
    * Note: `flip-link` is not strictly necessary for this example (it doesn't need
      stack protection), however it can be considered best practices to include it.
1. Install the cross-compile target: `rustup target add thumbv7em-none-eabihf`
1. Install the STLink drivers

## Build & Download to Board
1. Connect the board via USB
1. Optional: change your targeted platform in `Cargo.toml` and `.cargo/config` (it defaults to STM32F401RE)
1. Run `cargo run`
1. Enjoy your running program :)

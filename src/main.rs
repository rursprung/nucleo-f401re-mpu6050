//! Demonstrate the usage of the `MPU6050` crate, based on their README and a previous blink example.
#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use core::fmt::Write;

use panic_halt as _; // Halt on panic

use cortex_m_rt::entry;
use stm32f4xx_hal::{
    i2c::Mode,
    pac::{self},
    prelude::*,
    serial::config::Config,
};

use mpu6050::*;

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        // Set up the UART pin
        let gpioa = dp.GPIOA.split();
        let tx_pin = gpioa.pa2.into_alternate();
        let mut tx = dp
            .USART2
            .tx(
                tx_pin,
                Config::default()
                    .baudrate(115200.bps())
                    .wordlength_8()
                    .parity_none(),
                &clocks,
            )
            .unwrap();

        // Set up the I2C pins
        let gpiob = dp.GPIOB.split();
        let scl = gpiob.pb8;
        let sda = gpiob.pb9;

        // Create a delay abstraction based on SysTick
        let mut delay = cp.SYST.delay(&clocks);

        let i2c = dp.I2C1.i2c(
            (scl, sda),
            Mode::Standard {
                frequency: 100.kHz(),
            },
            &clocks,
        );

        let mut mpu = Mpu6050::new(i2c);
        mpu.init(&mut delay).unwrap();

        loop {
            // get roll and pitch estimate
            let acc = mpu.get_acc_angles().unwrap();
            writeln!(tx, "r/p: {:?}", acc).unwrap();

            // get temp
            let temp = mpu.get_temp().unwrap();
            writeln!(tx, "temp: {:?}c", temp).unwrap();

            // get gyro data, scaled with sensitivity
            let gyro = mpu.get_gyro().unwrap();
            writeln!(tx, "gyro: {:?}", gyro).unwrap();

            // get accelerometer data, scaled with sensitivity
            let acc = mpu.get_acc().unwrap();
            writeln!(tx, "acc: {:?}", acc).unwrap();

            delay.delay_ms(1000_u32);
        }
    }

    loop {}
}

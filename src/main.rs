//! Demonstrate the usage of the `MPU6050` crate, based on their README and a previous blink example.
#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use defmt_rtt as _;

use panic_halt as _; // Halt on panic

use cortex_m_rt::entry;
use stm32f4xx_hal::{
    i2c::Mode,
    pac::{self},
    prelude::*,
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
            defmt::info!("r/p: {}", defmt::Debug2Format(&acc));

            // get temp
            let temp = mpu.get_temp().unwrap();
            defmt::info!("temp: {}°C", defmt::Debug2Format(&temp));

            // get gyro data, scaled with sensitivity
            let gyro = mpu.get_gyro().unwrap();
            defmt::info!("gyro: {}", defmt::Debug2Format(&gyro));

            // get accelerometer data, scaled with sensitivity
            let acc = mpu.get_acc().unwrap();
            defmt::info!("acc: {}", defmt::Debug2Format(&acc));

            delay.delay_ms(1000_u32);
        }
    }

    loop {}
}

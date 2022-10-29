//! Demonstrate the usage of the `MPU6050` crate, based on their README and a previous blink example.
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;

use alloc::format;

use defmt_rtt as _;

use panic_halt as _; // Halt on panic

use cortex_m_rt::entry;
use stm32f4xx_hal::{
    i2c::Mode,
    pac::{self},
    prelude::*,
};

use mpu6050::*;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}

fn init_allocator() {
    // Initialize the allocator BEFORE you use it
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 1024;
    static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { ALLOCATOR.init(HEAP.as_ptr() as usize, HEAP_SIZE) }
}

#[entry]
fn main() -> ! {
    init_allocator();

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
            defmt::info!("{}", format!("r/p: {:?}", acc).as_str());

            // get temp
            let temp = mpu.get_temp().unwrap();
            defmt::info!("{}", format!("temp: {:?}°C", temp).as_str());

            // get gyro data, scaled with sensitivity
            let gyro = mpu.get_gyro().unwrap();
            defmt::info!("{}", format!("gyro: {:?}", gyro).as_str());

            // get accelerometer data, scaled with sensitivity
            let acc = mpu.get_acc().unwrap();
            defmt::info!("{}", format!("acc: {:?}", acc).as_str());

            delay.delay_ms(1000_u32);
        }
    }

    loop {}
}

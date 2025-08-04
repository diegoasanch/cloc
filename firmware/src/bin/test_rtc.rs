// This file is intended to be used to test the RTC driver on-device.
// It is not part of the main application.

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio;
use embassy_rp::i2c;
use embassy_rp::i2c::Config;
use embassy_rp::i2c::InterruptHandler;
use embassy_rp::peripherals::I2C1;
use embassy_time::Timer;
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

use cloc::rtc::Rtc;

bind_interrupts!(struct Irqs {
    I2C1_IRQ => InterruptHandler<I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);

    let sda = p.PIN_2;
    let scl = p.PIN_3;

    info!("set up i2c ");
    let i2c = i2c::I2c::new_async(p.I2C1, scl, sda, Irqs, Config::default());

    info!("Initializing RTC...");

    // Create RTC instance
    let mut rtc = Rtc::new(i2c);

    // Initialize the RTC
    match rtc.init() {
        Ok(()) => info!("RTC initialized successfully"),
        Err(_) => {
            error!("Failed to initialize RTC");
            return;
        }
    }

    // Test loop
    loop {
        led.set_high();

        // Test reading datetime
        match rtc.get_datetime() {
            Ok(datetime) => {
                info!(
                    "Current datetime: {:?}-{:?}-{:?} {:?}:{:?}:{:?}",
                    datetime.year,
                    datetime.month,
                    datetime.day,
                    datetime.hour,
                    datetime.minute,
                    datetime.second,
                );
            }
            Err(_) => {
                error!("Failed to read datetime");
            }
        }

        // Test reading temperature
        match rtc.get_temperature() {
            Ok(temp) => {
                info!("Temperature: {}°C", temp);
            }
            Err(_) => {
                info!("Temperature reading not supported");
            }
        }

        // Test checking busy status
        match rtc.is_busy() {
            Ok(busy) => {
                info!("RTC busy: {}", busy);
            }
            Err(_) => {
                error!("Failed to check busy status");
            }
        }

        led.set_low();

        // Wait 5 seconds before next reading
        Timer::after_secs(5).await;
    }
}

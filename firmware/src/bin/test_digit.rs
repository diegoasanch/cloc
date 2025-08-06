#![no_std]
#![no_main]

use cloc::{digit::Digit, tpic6b595::TPIC6B595};
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::{
    gpio,
    pwm::{Config, Pwm},
};
use embassy_time::{Duration, Timer};
use fixed::traits::ToFixed;
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Configure pins for TPIC6B595
    // You may need to adjust these pin numbers based on your hardware setup
    let srck = Output::new(p.PIN_18, Level::Low);
    let rck = Output::new(p.PIN_19, Level::Low);
    let ser_in = Output::new(p.PIN_17, Level::Low);

    // Configure PWM for OE (Output Enable) pin
    // PIN_16 is Channel A of PWM_SLICE0 according to the pin mapping
    let mut pwm_config = Config::default();
    pwm_config.top = 100; // Set top value for 1000 steps
    pwm_config.compare_a = 90; // 50% duty cycle (500/1000)
    pwm_config.divider = 125u16.to_fixed(); // Set divider for appropriate frequency
    pwm_config.enable = true;

    let _oe_pwm = Pwm::new_output_a(p.PWM_SLICE0, p.PIN_16, pwm_config);
    info!("PWM configured with 50% duty cycle");

    let mut tpic = TPIC6B595::new(srck, rck, ser_in);

    // Initialize the shift register
    tpic.init().await;
    info!("TPIC6B595 initialized");

    let mut digit;
    let mut dot = false;

    loop {
        for i in 0..10 {
            digit = Digit::new(i, dot, true);
            let binary = digit.to_binary();
            tpic.set_outputs(binary).await;
            Timer::after(Duration::from_millis(1000)).await;
            dot = !dot;
        }
    }
}

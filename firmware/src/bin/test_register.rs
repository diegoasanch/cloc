#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::pwm::{Config, Pwm};
use embassy_time::{Duration, Timer};
use fixed::traits::ToFixed;
use {defmt_rtt as _, panic_probe as _};

use cloc::tpic6b595::TPIC6B595;

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
    pwm_config.top = 1000; // Set top value for 1000 steps
    pwm_config.compare_a = 900; // 50% duty cycle (500/1000)
    pwm_config.divider = 125u16.to_fixed(); // Set divider for appropriate frequency
    pwm_config.enable = true;

    let _oe_pwm = Pwm::new_output_a(p.PWM_SLICE0, p.PIN_16, pwm_config);
    info!("PWM configured with 50% duty cycle");

    let mut tpic = TPIC6B595::new(srck, rck, ser_in);

    // Initialize the shift register
    tpic.init().await;
    info!("TPIC6B595 initialized");

    // Wait a moment before starting
    Timer::after(Duration::from_millis(1000)).await;

    loop {
        info!("=== Starting test cycle ===");

        // Step 1: Cycle through each bit individually
        info!("Step 1: Cycling through each bit individually");
        for bit in 0..8 {
            let pattern = 1 << bit;
            info!("Setting bit {} (pattern: 0b{:08b})", bit, pattern);
            tpic.set_outputs(pattern).await;
            Timer::after(Duration::from_millis(1000)).await;
        }

        // Step 2: Turn on bits one by one until all are on
        info!("Step 2: Turning on bits one by one");
        for bit in 0..8 {
            let pattern = (1 << (bit + 1)) - 1; // Creates pattern like 0b00000001, 0b00000011, 0b00000111, etc.
            info!("Adding bit {} (pattern: 0b{:08b})", bit, pattern);
            tpic.set_outputs(pattern).await;
            Timer::after(Duration::from_millis(1000)).await;
        }

        // Step 3: Turn off bits one by one until all are off
        info!("Step 3: Turning off bits one by one");
        for bit in (0..8).rev() {
            let pattern = (1 << bit) - 1; // Creates pattern like 0b01111111, 0b00111111, 0b00011111, etc.
            info!("Removing bit {} (pattern: 0b{:08b})", bit, pattern);
            tpic.set_outputs(pattern).await;
            Timer::after(Duration::from_millis(1000)).await;
        }

        // Clear all outputs
        info!("Clearing all outputs");
        tpic.clear_outputs().await;
        Timer::after(Duration::from_millis(1000)).await;

        info!("=== Test cycle complete, restarting in 2 seconds ===");
        Timer::after(Duration::from_millis(2000)).await;
    }
}

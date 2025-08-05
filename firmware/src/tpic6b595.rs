use embassy_rp::gpio::Output;
use embassy_time::{Duration, Timer};

/// TPIC6B595 8-bit shift register handler
///
/// This device contains an 8-bit serial-in, parallel-out shift register
/// that feeds an 8-bit D-type storage register. Data transfers through
/// the shift and storage registers on the rising edge of the shift-register
/// clock (SRCK) and the register clock (RCK), respectively.
///
/// Note: SRCLR is tied to 5V (always high) and G (output enable) is controlled
/// externally via PWM, so these pins are not managed by this handler.
pub struct TPIC6B595 {
    /// Shift Register Clock - data is shifted on rising edge
    srck: Output<'static>,
    /// Register Clock - data is transferred to storage register on rising edge
    rck: Output<'static>,
    /// Serial Input - data input pin
    ser_in: Output<'static>,
}

impl TPIC6B595 {
    /// Create a new TPIC6B595 handler with the required pins
    pub fn new(srck: Output<'static>, rck: Output<'static>, ser_in: Output<'static>) -> Self {
        Self { srck, rck, ser_in }
    }

    /// Initialize the shift register
    /// Sets all pins to their default states
    pub async fn init(&mut self) {
        // Set all pins to low initially
        self.srck.set_low();
        self.rck.set_low();
        self.ser_in.set_low();

        // Small delay to ensure stable state
        Timer::after(Duration::from_micros(10)).await;
    }

    /// Push a single bit to the shift register
    /// Data is shifted on the rising edge of SRCK
    async fn push_bit(&mut self, bit: bool) {
        // Set the data bit
        if bit {
            self.ser_in.set_high();
        } else {
            self.ser_in.set_low();
        }

        // Small delay to ensure data is stable
        Timer::after(Duration::from_micros(1)).await;

        // Rising edge of SRCK to shift the bit
        self.srck.set_high();
        Timer::after(Duration::from_micros(1)).await;
        self.srck.set_low();
    }

    /// Push an 8-bit value to the shift register
    /// Shifts the data MSB first (bit 7 down to bit 0)
    pub async fn push_byte(&mut self, data: u8) {
        // Push each bit, starting with the most significant bit
        for i in (0..8).rev() {
            let bit = (data >> i) & 0x01;
            self.push_bit(bit != 0).await;
        }
    }

    /// Transfer data from shift register to storage register
    /// Data is transferred on the rising edge of RCK
    pub async fn transfer_to_storage(&mut self) {
        // Rising edge of RCK to transfer data
        self.rck.set_high();
        Timer::after(Duration::from_micros(1)).await;
        self.rck.set_low();
    }

    /// Push a byte and immediately transfer it to storage
    /// This is a convenience method that combines push_byte and transfer_to_storage
    pub async fn push_byte_and_transfer(&mut self, data: u8) {
        self.push_byte(data).await;
        self.transfer_to_storage().await;
    }

    /// Push multiple bytes to the shift register
    /// Useful for cascading multiple devices
    pub async fn push_bytes(&mut self, data: &[u8]) {
        for &byte in data {
            self.push_byte(byte).await;
        }
    }

    /// Push multiple bytes and transfer them to storage
    /// This is a convenience method for multiple bytes
    pub async fn push_bytes_and_transfer(&mut self, data: &[u8]) {
        self.push_bytes(data).await;
        self.transfer_to_storage().await;
    }

    /// Set specific output pins by providing a bit pattern
    /// Each bit corresponds to an output (bit 0 = output 0, bit 7 = output 7)
    pub async fn set_outputs(&mut self, pattern: u8) {
        self.push_byte_and_transfer(pattern).await;
    }

    /// Clear all outputs (set them all to low)
    pub async fn clear_outputs(&mut self) {
        self.push_byte_and_transfer(0x00).await;
    }

    /// Set all outputs (set them all to high)
    pub async fn set_all_outputs(&mut self) {
        self.push_byte_and_transfer(0xFF).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embassy_rp::gpio::{Level, Output};

    #[test]
    fn test_tpic6b595_creation() {
        // This would need proper pin setup in a real test environment
        // For now, just verify the struct can be created
        let p = embassy_rp::init(Default::default());
        let _tpic = TPIC6B595::new(
            Output::new(p.PIN_0, Level::Low),
            Output::new(p.PIN_1, Level::Low),
            Output::new(p.PIN_2, Level::Low),
        );
    }
}

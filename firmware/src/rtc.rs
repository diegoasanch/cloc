use defmt::*;
use embedded_hal::i2c::I2c;

/// Simple datetime structure
#[derive(Debug, Clone, Copy)]
pub struct DateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

/// RTC driver for DS3231
pub struct Rtc<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> Rtc<I2C>
where
    I2C: I2c,
{
    /// Create a new DS3231 RTC instance
    pub fn new(i2c: I2C) -> Self {
        Self {
            i2c,
            address: 0x68, // DS3231 I2C address
        }
    }

    /// Initialize the RTC
    pub fn init(&mut self) -> Result<(), I2C::Error> {
        // Try to read the current time to verify communication
        match self.get_datetime() {
            Ok(_) => {
                info!("RTC initialized successfully");
                Ok(())
            }
            Err(e) => {
                error!("Failed to initialize RTC");
                Err(e)
            }
        }
    }

    /// Get the current datetime
    pub fn get_datetime(&mut self) -> Result<DateTime, I2C::Error> {
        // Read 7 bytes starting from register 0x00 (seconds)
        let mut data = [0u8; 7];
        self.i2c.write_read(self.address, &[0x00], &mut data)?;

        // Convert BCD to decimal
        let second = ((data[0] & 0x7F) >> 4) * 10 + (data[0] & 0x0F);
        let minute = ((data[1] & 0x7F) >> 4) * 10 + (data[1] & 0x0F);
        let hour = ((data[2] & 0x3F) >> 4) * 10 + (data[2] & 0x0F);
        let day = ((data[3] & 0x3F) >> 4) * 10 + (data[3] & 0x0F);
        let month = ((data[4] & 0x1F) >> 4) * 10 + (data[4] & 0x0F);
        let year = 2000 + (((data[5] >> 4) * 10 + (data[5] & 0x0F)) as u16); // only two digits are stored in the year register

        Ok(DateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }

    /// Set the datetime
    pub fn set_datetime(&mut self, datetime: &DateTime) -> Result<(), I2C::Error> {
        // Convert decimal to BCD
        let second_bcd = ((datetime.second / 10) << 4) | (datetime.second % 10);
        let minute_bcd = ((datetime.minute / 10) << 4) | (datetime.minute % 10);
        let hour_bcd = ((datetime.hour / 10) << 4) | (datetime.hour % 10);
        let day_bcd = ((datetime.day / 10) << 4) | (datetime.day % 10);
        let month_bcd = ((datetime.month / 10) << 4) | (datetime.month % 10);
        let year_remainder = (datetime.year % 100) as u8;
        let year_bcd = ((year_remainder / 10) << 4) | (year_remainder % 10);

        // Write to registers starting from 0x00
        let data = [
            0x00, second_bcd, minute_bcd, hour_bcd, day_bcd, month_bcd, year_bcd,
        ];
        self.i2c.write(self.address, &data)
    }

    /// Get the temperature reading from the RTC
    pub fn get_temperature(&mut self) -> Result<f32, I2C::Error> {
        // Read temperature registers (0x11 and 0x12)
        let mut data = [0u8; 2];
        self.i2c.write_read(self.address, &[0x11], &mut data)?;

        // Convert to temperature (DS3231 temperature format)
        let temp_raw = ((data[0] as i16) << 2) | ((data[1] >> 6) as i16);
        let temperature = (temp_raw as f32) * 0.25;

        Ok(temperature)
    }

    /// Check if the RTC is busy
    pub fn is_busy(&mut self) -> Result<bool, I2C::Error> {
        // Read status register (0x0F)
        let mut data = [0u8; 1];
        self.i2c.write_read(self.address, &[0x0F], &mut data)?;

        // Check BSY bit (bit 2)
        Ok((data[0] & 0x04) != 0)
    }
}

#[cfg(feature = "bmep280")]
use embedded_hal::i2c::I2c;

#[cfg(feature = "bmep280")]
use embedded_hal::delay::DelayNs;

#[cfg(feature = "bmep280")]
use crate::error::Error;

#[cfg(feature = "bmep280")]
mod registers {
    pub const ID: u8 = 0xD0;
    pub const RESET: u8 = 0xE0;
    pub const CTRL_HUM: u8 = 0xF2;
    pub const STATUS: u8 = 0xF3;
    pub const CTRL_MEAS: u8 = 0xF4;
    pub const CONFIG: u8 = 0xF5;
    //Data Registers
    pub const PRESS_MSB: u8 = 0xF7;
   //pub const PRESS_LSB: u8 = 0xF8;
   //pub const PRESS_XLSB: u8 = 0xF9;
   //pub const TEMP_MSB: u8 = 0xFA;
   //pub const TEMP_LSB: u8 = 0xFB;
   //pub const TEMP_XLSB: u8 = 0xFC;
   //pub const HUM_MSB: u8 = 0xFD;
   //pub const HUM_LSB: u8 = 0xFE;
    // Calibration Registers
    pub const CALIB00: u8 = 0x88;
    pub const CALIB26: u8 = 0xE1;
}

#[cfg(feature = "bmep280")]
use registers::*;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg(feature = "bmep280")]
pub enum DeviceType {
    Bme280,  // Chip ID 0x60 - has humidity
    Bmp280,  // Chip ID 0x58 - no humidity
}

#[cfg(feature = "bmep280")]
pub struct Bme280<I2C> {
    i2c: I2C,
    address: u8,
    device_type: DeviceType,
    calibration: CalibrationData,
    config: SensorConfig,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg(feature = "bmep280")]
pub enum Oversampling {
    Skipped,
    X1,
    X2,
    X4,
    X8,
    X16,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg(feature = "bmep280")]
pub enum Mode {
    Sleep,
    Forced,
    Normal,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg(feature = "bmep280")]
pub enum StandbyTime {
    Ms0_5,
    Ms62_5,
    Ms125,
    Ms250,
    Ms500,
    Ms1000,
    Ms10,
    Ms20,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg(feature = "bmep280")]
pub enum FilterCoefficient {
    Off,
    C2,
    C4,
    C8,
    C16,
}

#[derive(Debug, Clone, Copy)]
#[cfg(feature = "bmep280")]
struct CalibrationData {
    dig_t1: u16,
    dig_t2: i16,
    dig_t3: i16,
    dig_p1: u16,
    dig_p2: i16,
    dig_p3: i16,
    dig_p4: i16,
    dig_p5: i16,
    dig_p6: i16,
    dig_p7: i16,
    dig_p8: i16,
    dig_p9: i16,
    dig_h1: u8,
    dig_h2: i16,
    dig_h3: u8,
    dig_h4: i16,
    dig_h5: i16,
    dig_h6: i8,
}

#[derive(Debug, Clone, Copy)]
#[cfg(feature = "bmep280")]
struct SensorConfig {
    temp_oversampling: Oversampling,
    press_oversampling: Oversampling,
    hum_oversampling: Oversampling,
    mode: Mode,
    standby_time: StandbyTime,
    filter: FilterCoefficient,
}

#[cfg(feature = "bmep280")]
impl<I2C, E> Bme280<I2C>
where
    I2C: I2c<Error = E>,
{
    pub fn new(i2c: I2C, address: u8) -> Self {
        Bme280 {
            i2c,
            address,
            device_type: DeviceType::Bme280,
            calibration: CalibrationData {
                dig_t1: 0,
                dig_t2: 0,
                dig_t3: 0,
                dig_p1: 0,
                dig_p2: 0,
                dig_p3: 0,
                dig_p4: 0,
                dig_p5: 0,
                dig_p6: 0,
                dig_p7: 0,
                dig_p8: 0,
                dig_p9: 0,
                dig_h1: 0,
                dig_h2: 0,
                dig_h3: 0,
                dig_h4: 0,
                dig_h5: 0,
                dig_h6: 0,
            },
            config: SensorConfig {
                temp_oversampling: Oversampling::X1,
                press_oversampling: Oversampling::X1,
                hum_oversampling: Oversampling::X1,
                mode: Mode::Sleep,
                standby_time: StandbyTime::Ms0_5,
                filter: FilterCoefficient::Off,
            },
        }
    }

    pub fn verify_identity(&mut self) -> Result<DeviceType, Error<E>> {
        let mut buffer = [0u8];
        self.i2c.write_read(self.address, &[ID], &mut buffer)?;

        match buffer[0] {
            0x60 => {
                self.device_type = DeviceType::Bme280;
                Ok(DeviceType::Bme280)
            },
            0x58 | 0x56 | 0x57 => {
                self.device_type = DeviceType::Bmp280;
                Ok(DeviceType::Bmp280)
            },
            _ => Err(Error::NotDetected)
        }
    }

    pub fn get_device_type(&self) -> DeviceType {
        self.device_type
    }

    pub fn reset<D>(&mut self, delay: &mut D) -> Result<(), Error<E>> 
        where 
            D: DelayNs
    {
        self.i2c.write(self.address, &[RESET, 0xB6])?;
        delay.delay_ms(10);
        Ok(())
    }

    fn read_calibration_data<D>(&mut self, delay: &mut D) -> Result<(), Error<E>> 
        where 
            D: DelayNs
    {
        // Read Temperature and Pressure Calibration (both BME280 and BMP280)
        delay.delay_ms(5);
        let mut calib_data = [0u8; 26];
        self.i2c
            .write_read(self.address, &[CALIB00], &mut calib_data)?;
        self.calibration.dig_t1 = u16::from_le_bytes([calib_data[0], calib_data[1]]);
        self.calibration.dig_t2 = i16::from_le_bytes([calib_data[2], calib_data[3]]);
        self.calibration.dig_t3 = i16::from_le_bytes([calib_data[4], calib_data[5]]);
        self.calibration.dig_p1 = u16::from_le_bytes([calib_data[6], calib_data[7]]);
        self.calibration.dig_p2 = i16::from_le_bytes([calib_data[8], calib_data[9]]);
        self.calibration.dig_p3 = i16::from_le_bytes([calib_data[10], calib_data[11]]);
        self.calibration.dig_p4 = i16::from_le_bytes([calib_data[12], calib_data[13]]);
        self.calibration.dig_p5 = i16::from_le_bytes([calib_data[14], calib_data[15]]);
        self.calibration.dig_p6 = i16::from_le_bytes([calib_data[16], calib_data[17]]);
        self.calibration.dig_p7 = i16::from_le_bytes([calib_data[18], calib_data[19]]);
        self.calibration.dig_p8 = i16::from_le_bytes([calib_data[20], calib_data[21]]);
        self.calibration.dig_p9 = i16::from_le_bytes([calib_data[22], calib_data[23]]);
        self.calibration.dig_h1 = calib_data[25];

        if self.calibration.dig_t1 == 0 {
            return Err(Error::CalibrationFailed);
        }
        // Read humidity calibration ONLY for BME280
        if self.device_type == DeviceType::Bme280 {
            let mut hum_calib = [0u8; 7];
            self.i2c
                .write_read(self.address, &[CALIB26], &mut hum_calib)?;
            self.calibration.dig_h2 = i16::from_le_bytes([hum_calib[0], hum_calib[1]]);
            self.calibration.dig_h3 = hum_calib[2];
            self.calibration.dig_h4 = (hum_calib[3] as i16) << 4 | (hum_calib[4] as i16 & 0x0F);
            self.calibration.dig_h5 = (hum_calib[5] as i16) << 4 | ((hum_calib[4] as i16) >> 4);
            self.calibration.dig_h6 = hum_calib[6] as i8;
        }

        Ok(())
    }

    pub fn validate_calibration(&self) -> bool {
        self.calibration.dig_t1 != 0 && self.calibration.dig_p1 != 0
    }

    pub fn configure(
        &mut self,
        temp_oversampling: Oversampling,
        press_oversampling: Oversampling,
        hum_oversampling: Oversampling,
        mode: Mode,
        standby_time: StandbyTime,
        filter: FilterCoefficient,
    ) -> Result<(), Error<E>> {
        self.config = SensorConfig {
            temp_oversampling,
            press_oversampling,
            hum_oversampling,
            mode,
            standby_time,
            filter,
        };

        // Configure humidity control ONLY for BME280
        if self.device_type == DeviceType::Bme280 {
            let hum_ctrl = match hum_oversampling {
                Oversampling::Skipped => 0x00,
                Oversampling::X1 => 0x01,
                Oversampling::X2 => 0x02,
                Oversampling::X4 => 0x03,
                Oversampling::X8 => 0x04,
                Oversampling::X16 => 0x05,
            };
            self.i2c.write(self.address, &[CTRL_HUM, hum_ctrl])?;
        }

        //Configure temperature control
        let temp_ctrl = match temp_oversampling {
            Oversampling::Skipped => 0x00,
            Oversampling::X1 => 0x20,
            Oversampling::X2 => 0x40,
            Oversampling::X4 => 0x60,
            Oversampling::X8 => 0x80,
            Oversampling::X16 => 0xA0,
        };
        let press_ctrl = match press_oversampling {
            Oversampling::Skipped => 0x00,
            Oversampling::X1 => 0x04,
            Oversampling::X2 => 0x08,
            Oversampling::X4 => 0x0C,
            Oversampling::X8 => 0x10,
            Oversampling::X16 => 0x14,
        };
        let mode_ctrl = match mode {
            Mode::Sleep => 0x00,
            Mode::Forced => 0x01,
            Mode::Normal => 0x03,
        };
        let ctrl_meas = temp_ctrl | press_ctrl | mode_ctrl;
        self.i2c.write(self.address, &[CTRL_MEAS, ctrl_meas])?;
        // Configure standby time and filter
        let standby_bits = match standby_time {
            StandbyTime::Ms0_5 => 0x00,
            StandbyTime::Ms62_5 => 0x01,
            StandbyTime::Ms125 => 0x02,
            StandbyTime::Ms250 => 0x03,
            StandbyTime::Ms500 => 0x04,
            StandbyTime::Ms1000 => 0x05,
            StandbyTime::Ms10 => 0x06,
            StandbyTime::Ms20 => 0x07,
        };
        let filter_bits = match filter {
            FilterCoefficient::Off => 0x00,
            FilterCoefficient::C2 => 0x01,
            FilterCoefficient::C4 => 0x02,
            FilterCoefficient::C8 => 0x03,
            FilterCoefficient::C16 => 0x04,
        };
        let config = (standby_bits << 5) | (filter_bits << 2);
        self.i2c.write(self.address, &[CONFIG, config])?;
        Ok(())
    }

    pub fn initialize_sensor<D>(
        &mut self,
        delay: &mut D,
        temp_oversampling: Oversampling,
        press_oversampling: Oversampling,
        hum_oversampling: Oversampling,
        mode: Mode,
        standby_time: StandbyTime,
        filter: FilterCoefficient,
    ) -> Result<DeviceType, Error<E>> 
        where 
            D: DelayNs
    {

        //Verify device identity
        let device_type = self.verify_identity()?;

        // Reset sensor
        self.reset(delay)?;

        // Read calibration data
        self.read_calibration_data(delay)?;

        // Validate calibration
        if !self.validate_calibration() {
            return Err(Error::CalibrationFailed);
        }

        self.configure(
            temp_oversampling,
            press_oversampling,
            hum_oversampling,
            mode,
            standby_time,
            filter,
        )?;
        Ok(device_type)
    }

    pub fn is_measuring(&mut self) -> Result<bool, Error<E>> {
        let mut buffer = [0u8];
        self.i2c.write_read(self.address, &[STATUS], &mut buffer)?;
        Ok((buffer[0] & 0x08) != 0)
    }

    pub fn is_updating(&mut self) -> Result<bool, Error<E>> {
        let mut buffer = [0u8];
        self.i2c.write_read(self.address, &[STATUS], &mut buffer)?;
        Ok((buffer[0] & 0x01) != 0)
    }

    fn read_raw_data(&mut self) -> Result<[i32; 8], Error<E>> {
        // Remove all the triggering code from here - let read_all() handle it
        let mut buffer = [0u8; 8];
        self.i2c
            .write_read(self.address, &[PRESS_MSB], &mut buffer)?;
        let press_raw =
            ((buffer[0] as i32) << 12) | ((buffer[1] as i32) << 4) | ((buffer[2] as i32) >> 4);
        let temp_raw =
            ((buffer[3] as i32) << 12) | ((buffer[4] as i32) << 4) | ((buffer[5] as i32) >> 4);
        let hum_raw = ((buffer[6] as i32) << 8) | buffer[7] as i32;

        if press_raw == 0x80000 || temp_raw == 0x80000 {
            return Err(Error::NotReady);
        }

        Ok([press_raw, temp_raw, hum_raw, 0, 0, 0, 0, 0])
    }

    fn compensate_temperature(&self, raw_temp: i32) -> (f32, i32) {
        let var1 = (((raw_temp >> 3) - ((self.calibration.dig_t1 as i32) << 1))
            * (self.calibration.dig_t2 as i32))
            >> 11;
        let var2 = (((((raw_temp >> 4) - (self.calibration.dig_t1 as i32))
                    * ((raw_temp >> 4) - (self.calibration.dig_t1 as i32)))
                >> 12)
            * (self.calibration.dig_t3 as i32))
            >> 14;
        let t_fine = var1 + var2;
        let temperature = (t_fine * 5 + 128) >> 8;
        (temperature as f32 / 100.0, t_fine)
    }

    fn compensate_pressure(&self, raw_press: i32, t_fine: i32) -> f32 {
        let mut var1 = (t_fine >> 1) - 64000;
        let mut var2 = (((var1 >> 2) * (var1 >> 2)) >> 11) * (self.calibration.dig_p6 as i32);
        var2 = var2 + ((var1 * (self.calibration.dig_p5 as i32)) << 1);
        var2 = (var2 >> 2) + ((self.calibration.dig_p4 as i32) << 16);
        var1 = (((self.calibration.dig_p3 as i32) * (((var1 >> 2) * (var1 >> 2)) >> 13)) >> 3)
            + (((self.calibration.dig_p2 as i32) * var1) >> 1);
        var1 = var1 >> 18;
        var1 = ((32768 + var1) + (self.calibration.dig_p1 as i32)) >> 15;
        if var1 == 0 {
            return 0.0;
        }
        let mut pressure = ((1048576 - raw_press) - (var2 >> 12)) * 3125;
        if (pressure as i64) < 0x80000000 {
            pressure = (pressure << 1) / var1;
        } else {
            pressure = (pressure / var1) << 1;
        }
        var1 =
            ((self.calibration.dig_p9 as i32) * (((pressure >> 3) * (pressure >> 3)) >> 13)) >> 12;
        var2 = ((pressure >> 2) * (self.calibration.dig_p8 as i32)) >> 13;
        pressure = pressure + ((var1 + var2 + (self.calibration.dig_p7 as i32)) >> 4);
        pressure as f32 / 100.
    }

    fn compensate_humidity(&self, raw_hum: i32, t_fine: i32) -> f32 {
        let var1 = t_fine - 76800;
        let var2 = (raw_hum << 14)
            - ((self.calibration.dig_h4 as i32) << 20)
            - ((self.calibration.dig_h5 as i32) * var1);
        let var3 = var2 + 16384;
        let var4 = var3 >> 15;
        let var5 = ((((var4 * (self.calibration.dig_h6 as i32)) >> 10)
                * (((var4 * (self.calibration.dig_h3 as i32)) >> 11) + 32768))
            >> 10)
            + 2097152;
        let var6 = var5 * (self.calibration.dig_h2 as i32);
        let var7 = ((var6 >> 11) + 32768) >> 10;
        let var8 = var4 * (self.calibration.dig_h1 as i32);
        let var9 = (var8 >> 10) * (var8 >> 10);
        let var10 = (var9 >> 7) * (self.calibration.dig_h3 as i32);
        let var11 = (var8 * (self.calibration.dig_h2 as i32) + var10) >> 12;
        let var12 = var11 >> 4;
        let var13 = (var7 * var12) >> 13;
        let var14 = (var4 * (self.calibration.dig_h1 as i32)) >> 10;
        let var15 = (var14 + 32768) >> 10;
        let var16 = var13 * var15;
        let var17 = var16 >> 12;
        let humidity = var17 >> 12;
        humidity as f32 / 1024.0
    }

    pub fn read_all<D>(&mut self, delay: &mut D) -> Result<(f32, f32, Option<f32>), Error<E>> 
        where 
            D: DelayNs
    {
        if self.config.mode == Mode::Forced {
            self.trigger_measurement()?;

            let mut timeout_count = 0;
            while self.is_measuring()? {
                delay.delay_ms(1);
                timeout_count += 1;
                if timeout_count > 100 {
                    return Err(Error::Timeout);
                }
            }
        } else {
            delay.delay_ms(10);
        }
        let raw_data = self.read_raw_data()?;
        let (temperature, t_fine) = self.compensate_temperature(raw_data[1]);
        let pressure = self.compensate_pressure(raw_data[0], t_fine);

        let humidity = if self.device_type == DeviceType::Bme280 {
            Some(self.compensate_humidity(raw_data[2], t_fine))
        } else {
            None
        };

        Ok((temperature, pressure, humidity))
    }

    pub fn read_temperature(&mut self) -> Result<f32, Error<E>> {
        let raw_data = self.read_raw_data()?;
        let (temperature, _) = self.compensate_temperature(raw_data[1]);
        Ok(temperature)
    }

    pub fn read_pressure(&mut self) -> Result<f32, Error<E>> {
        let raw_data = self.read_raw_data()?;
        let (_, t_fine) = self.compensate_temperature(raw_data[1]);
        let pressure = self.compensate_pressure(raw_data[0], t_fine);
        Ok(pressure)
    }

    pub fn read_humidity(&mut self) -> Result<f32, Error<E>> {
        if self.device_type == DeviceType::Bmp280 {
            return Err(Error::NotSupported);
        }

        let raw_data = self.read_raw_data()?;
        let (_, t_fine) = self.compensate_temperature(raw_data[1]);
        let humidity = self.compensate_humidity(raw_data[2], t_fine);
        Ok(humidity)
    }

    pub fn trigger_measurement(&mut self) -> Result<(), Error<E>> {
        if self.config.mode == Mode::Forced {
            // Build the complete CTRL_MEAS value like configure() does
            let temp_ctrl = match self.config.temp_oversampling {
                Oversampling::Skipped => 0x00,
                Oversampling::X1 => 0x20,
                Oversampling::X2 => 0x40,
                Oversampling::X4 => 0x60,
                Oversampling::X8 => 0x80,
                Oversampling::X16 => 0xA0,
            };
            let press_ctrl = match self.config.press_oversampling {
                Oversampling::Skipped => 0x00,
                Oversampling::X1 => 0x04,
                Oversampling::X2 => 0x08,
                Oversampling::X4 => 0x0C,
                Oversampling::X8 => 0x10,
                Oversampling::X16 => 0x14,
            };

            // Force mode = 0x01
            let ctrl_meas = temp_ctrl | press_ctrl | 0x01;

            self.i2c.write(self.address, &[CTRL_MEAS, ctrl_meas])?;
        }
        Ok(())
    }

    pub fn set_mode(&mut self, mode: Mode) -> Result<(), Error<E>> {
        self.configure(
            self.config.temp_oversampling,
            self.config.press_oversampling,
            self.config.hum_oversampling,
            mode,
            self.config.standby_time,
            self.config.filter,
        )
    }

    pub fn has_humidity(&self) -> bool {
        self.device_type == DeviceType::Bme280
    }
}

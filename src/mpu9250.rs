#[cfg(feature = "mpu9250")]
use embedded_hal::i2c::I2c;
use crate::error::Error;

const WHO_AM_I: u8 = 0x75;
const WHO_AM_I_VALUE: u8 = 0x74;
const PWR_MGMT_1: u8 = 0x6B;
const ACCEL_CONFIG: u8 = 0x1C;
const GYRO_CONFIG: u8 = 0x1B;
const ACCEL_XOUT_H: u8 = 0x3B;
const TEMP_OUT_H: u8 = 0x41;
const GYRO_XOUT_H: u8 = 0x43;
const SMPRT_DIV: u8 = 0x19;
const CONFIG: u8 = 0x1A;

#[cfg_attr(docsrs, doc(cfg(feature = "mpu9250")))]
pub struct Mpu9250<I2C> {
    i2c: I2C,
    address: u8,
    accel_scale: f32,
    gyro_scale: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(docsrs, doc(cfg(feature = "mpu9250")))]
pub enum AccelRange {
    Range2G,
    Range4G,
    Range8G,
    Range16G,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(docsrs, doc(cfg(feature = "mpu9250")))]
pub enum GyroRange {
    Range250Dps,
    Range500Dps,
    Range1000Dps,
    Range2000Dps,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(docsrs, doc(cfg(feature = "mpu9250")))]
pub enum DlpfConfig {
    Bandwidth260Hz,
    Bandwidth184Hz,
}

impl<I2C, E> Mpu9250<I2C>
where 
    I2C: I2c<Error = E>
{
    pub fn new(i2c: I2C, address: u8) -> Self {
        Mpu9250 {
            i2c,
            address,
            accel_scale: 0.0,
            gyro_scale: 0.0,
        }
    }

    pub fn verify_identity(&mut self) -> Result<(), Error<E>> {
        let mut buffer = [0u8];
        self.i2c.write_read(self.address, &[WHO_AM_I], &mut buffer)?;
        if buffer[0] != WHO_AM_I_VALUE {
            return Err(Error::NotDetected);
        }
        Ok(())
    }

    pub fn configure_power(&mut self) -> Result<(), Error<E>> {
        let config = 0x01;
        self.i2c.write(self.address, &[PWR_MGMT_1, config])?;
        Ok(())
    }

    pub fn setup_accelerometer(&mut self, range: AccelRange) -> Result<(), Error<E>> {
        let (config_value, scale) = match range {
            AccelRange::Range2G => (0x00, 2.0 / 32768.0),
            AccelRange::Range4G => (0x08, 4.0 / 32768.0),
            AccelRange::Range8G => (0x10, 8.0 / 32768.0),
            AccelRange::Range16G => (0x18, 16.0 / 32768.0),
        };
        self.i2c.write(self.address, &[ACCEL_CONFIG, config_value])?;
        self.accel_scale = scale;
        Ok(())
    }

    pub fn setup_gyroscope(&mut self, range: GyroRange) -> Result<(), Error<E>> {
        let (config_value, scale) = match range {
            GyroRange::Range250Dps => (0x00, 250.0 / 32768.0),
            GyroRange::Range500Dps => (0x08, 500.0 / 32768.0),
            GyroRange::Range1000Dps => (0x10, 1000.0 / 32768.0),
            GyroRange::Range2000Dps => (0x18, 2000.0 / 32768.0),
        };
        self.i2c.write(self.address, &[GYRO_CONFIG, config_value])?;
        self.gyro_scale = scale;
        Ok(())
    }

    pub fn initialize_sensor(&mut self, accel_range: AccelRange, gyro_range: GyroRange) -> Result<(), Error<E>> {
        self.verify_identity()?;
        self.configure_power()?;
        self.setup_accelerometer(accel_range)?;
        self.setup_gyroscope(gyro_range)?;
        Ok(())
    }
    
    pub fn read_accel_raw(&mut self) -> Result<[i16; 3], Error<E>> {
        let mut buffer = [0u8; 6];
        self.i2c.write_read(self.address, &[ACCEL_XOUT_H], &mut buffer)?;
        let x = ((buffer[0] as i16) << 8) | buffer[1] as i16;
        let y = ((buffer[2] as i16) << 8) | buffer[3] as i16;
        let z = ((buffer[4] as i16) << 8) | buffer[5] as i16;
        Ok([x, y, z])
    }

    pub fn read_gyro_raw(&mut self) -> Result<[i16; 3], Error<E>> {
        let mut buffer = [0u8; 6];
        self.i2c.write_read(self.address, &[GYRO_XOUT_H], &mut buffer)?;
        let x = ((buffer[0] as i16) << 8) | buffer[1] as i16;
        let y = ((buffer[2] as i16) << 8) | buffer[3] as i16;
        let z = ((buffer[4] as i16) << 8) | buffer[5] as i16;
        Ok([x, y, z])
    }

    pub fn read_temp_raw(&mut self) -> Result<i16, Error<E>> {
        let mut buffer = [0u8; 2];
        self.i2c.write_read(self.address, &[TEMP_OUT_H], &mut buffer)?;
        let temp = ((buffer[0] as i16) << 8) | buffer[1] as i16;
        Ok(temp)
    }

    pub fn read_acceleration(&mut self) -> Result<[f32; 3], Error<E>> {
        let raw = self.read_accel_raw()?;
        let x = raw[0] as f32 * self.accel_scale;
        let y = raw[1] as f32 * self.accel_scale;
        let z = raw[2] as f32 * self.accel_scale;
        Ok([x, y, z])
    }

    pub fn read_angular_velocity(&mut self) -> Result<[f32; 3], Error<E>> {
        let raw = self.read_gyro_raw()?;
        let x = raw[0] as f32 * self.gyro_scale;
        let y = raw[1] as f32 * self.gyro_scale;
        let z = raw[2] as f32 * self.gyro_scale;
        Ok([x, y, z])
    }

    pub fn read_temperature_celsius(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_temp_raw()?;
        let temperature = (raw as f32) / 340.0 + 36.53;
        Ok(temperature)
    }

    pub fn set_sample_rate(&mut self, divider: u8) -> Result<(), Error<E>> {
        self.i2c.write(self.address, &[SMPRT_DIV, divider])?;
        Ok(())
    }

    pub fn set_dlpf_config(&mut self, config: DlpfConfig) -> Result<(), Error<E>> {
        let config_value = match config {
            DlpfConfig::Bandwidth260Hz => 0x00,
            DlpfConfig::Bandwidth184Hz => 0x01,
        };
        self.i2c.write(self.address, &[CONFIG, config_value])?;
        Ok(())
    }

    pub fn enter_sleep_mode(&mut self) -> Result<(), Error<E>> {
        let mut buffer = [0u8];
        self.i2c.write(self.address, &[PWR_MGMT_1])?;
        self.i2c.read(self.address, &mut buffer)?;
        let new_config = buffer[0] | 0x40;
        self.i2c.write(self.address, &[PWR_MGMT_1, new_config])?;
        Ok(())
    }

    pub fn wake_up(&mut self) -> Result<(), Error<E>> {
        let mut buffer = [0u8];
        self.i2c.write(self.address, &[PWR_MGMT_1])?;
        self.i2c.read(self.address, &mut buffer)?;
        let new_config = buffer[0] & 0xBF;
        self.i2c.write(self.address, &[PWR_MGMT_1, new_config])?;
        Ok(())
    }
}

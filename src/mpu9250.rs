//! MPU9250 9-axis motion tracking device driver.
//!
//! This module provides a driver for the MPU9250 sensor, which combines a
//! 3-axis accelerometer, 3-axis gyroscope, and 3-axis magnetometer.
//!
//! # Features
//!
//! - Accelerometer data reading in g-force units
//! - Gyroscope data reading in degrees per second
//! - Temperature reading in Celsius
//! - Configurable measurement ranges
//! - Power management (sleep/wake modes)
//! - Digital low-pass filter configuration
//!
//! # Examples
//!
//! ```rust,no_run
//! use hayasen::mpu9250::{Mpu9250, AccelRange, GyroRange};
//! use embedded_hal::i2c::I2c;
//!
//! # fn main() -> Result<(), hayasen::Error<()>> {
//! // Initialize sensor
//! let mut mpu = Mpu9250::new(i2c, 0x68);
//! mpu.initialize_sensor(AccelRange::Range2G, GyroRange::Range250Dps)?;
//!
//! // Read sensor data
//! let acceleration = mpu.read_acceleration()?;
//! let gyroscope = mpu.read_angular_velocity()?;
//! let temperature = mpu.read_temperature_celsius()?;
//!
//! println!("Acceleration: {:?} g", acceleration);
//! println!("Gyroscope: {:?} dps", gyroscope);
//! println!("Temperature: {:.1}°C", temperature);
//! # Ok(())
//! # }
//! ```

#[cfg(feature = "mpu9250")]
use embedded_hal::i2c::I2c;
use crate::error::Error;

// Register addresses
const WHO_AM_I: u8 = 0x75;
const WHO_AM_I_VALUE: u8 = 0x71; // Expected value for MPU9250
const PWR_MGMT_1: u8 = 0x6B;
const ACCEL_CONFIG: u8 = 0x1C;
const GYRO_CONFIG: u8 = 0x1B;
const ACCEL_XOUT_H: u8 = 0x3B;
const TEMP_OUT_H: u8 = 0x41;
const GYRO_XOUT_H: u8 = 0x43;
const SMPRT_DIV: u8 = 0x19;
const CONFIG: u8 = 0x1A;

/// MPU9250 9-axis motion tracking device driver.
///
/// Provides access to accelerometer, gyroscope, and temperature data
/// from the InvenSense MPU9250 sensor.
///
/// # Type Parameters
///
/// - `I2C`: Type implementing the `I2c` trait for communication
///
/// # Examples
///
/// ```rust,no_run
/// use hayasen::mpu9250::Mpu9250;
///
/// let mut mpu = Mpu9250::new(i2c_peripheral, 0x68);
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "mpu9250")))]
pub struct Mpu9250<I2C> {
    i2c: I2C,
    address: u8,
    accel_scale: f32,   // Scaling factor for acceleration
    gyro_scale: f32,    // Scaling factor for gyroscope
}

/// Accelerometer full-scale range configuration.
///
/// Determines the measurement range and sensitivity of the accelerometer.
/// Larger ranges provide lower resolution but can measure greater forces.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(docsrs, doc(cfg(feature = "mpu9250")))]
pub enum AccelRange {
    /// ±2g range with highest sensitivity
    Range2G,
    /// ±4g range
    Range4G,
    /// ±8g range
    Range8G,
    /// ±16g range with lowest sensitivity
    Range16G,
}

/// Gyroscope full-scale range configuration.
///
/// Determines the measurement range and sensitivity of the gyroscope.
/// Larger ranges provide lower resolution but can measure higher rotation rates.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(docsrs, doc(cfg(feature = "mpu9250")))]
pub enum GyroRange {
    /// ±250 degrees per second with highest sensitivity
    Range250Dps,
    /// ±500 degrees per second
    Range500Dps,
    /// ±1000 degrees per second
    Range1000Dps,
    /// ±2000 degrees per second with lowest sensitivity
    Range2000Dps,
}

/// Digital Low-Pass Filter (DLPF) configuration.
///
/// Controls the bandwidth of the internal digital low-pass filter,
/// affecting the noise and response characteristics of the sensor.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(docsrs, doc(cfg(feature = "mpu9250")))]
pub enum DlpfConfig {
    /// 260Hz bandwidth, 256Hz sample rate
    Bandwidth260Hz,
    /// 184Hz bandwidth, 188Hz sample rate
    Bandwidth184Hz,
}

impl<I2C, E> Mpu9250<I2C>
where 
    I2C: I2c<Error = E>
{
    /// Creates a new MPU9250 driver instance.
    ///
    /// # Arguments
    ///
    /// * `i2c` - I2C peripheral implementing the `I2c` trait
    /// * `address` - I2C address of the MPU9250 sensor (typically 0x68 or 0x69)
    ///
    /// # Returns
    ///
    /// A new `Mpu9250` instance. Call `initialize_sensor()` before reading data.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use hayasen::mpu9250::Mpu9250;
    ///
    /// let mpu = Mpu9250::new(i2c, 0x68);
    /// ```
    pub fn new(i2c: I2C, address: u8) -> Self {
        Mpu9250 {
            i2c,
            address,
            accel_scale: 0.0,     // Will be set during configuration 
            gyro_scale: 0.0,     // Will be set during configuration
        }
    }

    /// Verifies the sensor's identity by reading the WHO_AM_I register.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the sensor responds with the expected MPU9250 ID
    /// - `Err(Error::NotDetected)` if the sensor doesn't respond or ID doesn't match
    ///
    /// # Errors
    ///
    /// Returns `Error::I2c` if I2C communication fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use hayasen::mpu9250::Mpu9250;
    /// # let mut mpu = Mpu9250::new(i2c, 0x68);
    /// match mpu.verify_identity() {
    ///     Ok(()) => println!("MPU9250 detected successfully"),
    ///     Err(e) => println!("Sensor detection failed: {:?}", e),
    /// }
    /// ```
    pub fn verify_identity(&mut self) -> Result<(), Error<E>> {
        let mut buffer = [0u8];

        self.i2c.write(self.address, &[WHO_AM_I])?;
        self.i2c.read(self.address, &mut buffer)?;

        if buffer[0] != WHO_AM_I_VALUE {
            return Err(Error::NotDetected);
        }

        Ok(())
    }

    /// Configures power management settings.
    ///
    /// Wakes up the device and selects the internal oscillator as clock source.
    ///
    /// # Errors
    ///
    /// Returns `Error::I2c` if I2C communication fails.
    pub fn configure_power(&mut self) -> Result<(), Error<E>> {
        let config = 0x01; // Wake up and use internal oscillator
        self.i2c.write(self.address, &[PWR_MGMT_1, config])?;
        Ok(())
    }

    /// Configures the accelerometer measurement range.
    ///
    /// # Arguments
    ///
    /// * `range` - Desired full-scale range (`AccelRange`)
    ///
    /// # Errors
    ///
    /// Returns `Error::I2c` if I2C communication fails.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    pub fn setup_accelerometer(&mut self, range: AccelRange) -> Result<(), Error<E>> {
        let (config_value, scale) = match range {
            AccelRange::Range2G => (0x00, 2.0 / 32768.0),  // ±2g scale factor
            AccelRange::Range4G => (0x08, 4.0 / 32768.0),  // ±4g scale factor
            AccelRange::Range8G => (0x10, 8.0 / 32768.0),  // ±8g scale factor (fixed from 2.0)
            AccelRange::Range16G => (0x18, 16.0 / 32768.0),  // ±16g scale factor
        };

        self.i2c.write(self.address, &[ACCEL_CONFIG, config_value])?;
        self.accel_scale = scale;
        Ok(())
    }

    /// Configures the gyroscope measurement range.
    ///
    /// # Arguments
    ///
    /// * `range` - Desired full-scale range (`GyroRange`)
    ///
    /// # Errors
    ///
    /// Returns `Error::I2c` if I2C communication fails.
    pub fn setup_gyroscope(&mut self, range: GyroRange) -> Result<(), Error<E>> {
        let (config_value, scale) = match range {
            GyroRange::Range250Dps => (0x00, 250.0 / 32768.0),     // ±250 Dps
            GyroRange::Range500Dps => (0x08, 500.0 / 32768.0),     // ±500 Dps
            GyroRange::Range1000Dps => (0x10, 1000.0 / 32768.0),   // ±1000 Dps
            GyroRange::Range2000Dps => (0x18, 2000.0 / 32768.0),   // ±2000 Dps
        };

        self.i2c.write(self.address, &[GYRO_CONFIG, config_value])?;
        self.gyro_scale = scale;
        Ok(())
    }

    /// Comprehensive sensor initialization routine.
    ///
    /// Performs all necessary steps to configure the sensor for operation:
    /// 1. Verifies sensor identity
    /// 2. Configures power management
    /// 3. Sets up accelerometer range
    /// 4. Sets up gyroscope range
    ///
    /// # Arguments
    ///
    /// * `accel_range` - Desired accelerometer range
    /// * `gyro_range` - Desired gyroscope range
    ///
    /// # Errors
    ///
    /// Returns `Error::I2c` if I2C communication fails.
    /// Returns `Error::NotDetected` if sensor identity verification fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use hayasen::mpu9250::{Mpu9250, AccelRange, GyroRange};
    ///
    /// # let mut mpu = Mpu9250::new(i2c, 0x68);
    /// mpu.initialize_sensor(AccelRange::Range2G, GyroRange::Range250Dps)?;
    /// ```
    pub fn initialize_sensor(&mut self, accel_range: AccelRange, gyro_range: GyroRange) -> Result<(), Error<E>> {
        self.verify_identity()?;
        self.configure_power()?;
        self.setup_accelerometer(accel_range)?;
        self.setup_gyroscope(gyro_range)?;
        Ok(())
    }
    
    /// Reads raw accelerometer data as 16-bit integers.
    ///
    /// Returns raw sensor values before scaling is applied.
    /// Use `read_acceleration()` for values in g-force units.
    ///
    /// # Returns
    ///
    /// `[x, y, z]` array of raw accelerometer values
    ///
    /// # Errors
    ///
    /// Returns `Error::I2c` if I2C communication fails.
    pub fn read_accel_raw(&mut self) -> Result<[i16; 3], Error<E>> {
        let mut buffer = [0u8; 6];
        self.i2c.write(self.address, &[ACCEL_XOUT_H])?;
        self.i2c.read(self.address, &mut buffer)?;

        let x = ((buffer[0] as i16) << 8) | buffer[1] as i16;
        let y = ((buffer[2] as i16) << 8) | buffer[3] as i16;
        let z = ((buffer[4] as i16) << 8) | buffer[5] as i16;

        Ok([x, y, z])
    }

    /// Reads raw gyroscope data as 16-bit integers.
    ///
    /// Returns raw sensor values before scaling is applied.
    /// Use `read_angular_velocity()` for values in degrees per second.
    ///
    /// # Errors
    ///
    /// Returns `Error::I2c` if I2C communication fails.
    pub fn read_gyro_raw(&mut self) -> Result<[i16; 3], Error<E>> {
        let mut buffer = [0u8; 6];
        self.i2c.write(self.address, &[GYRO_XOUT_H])?;
        self.i2c.read(self.address, &mut buffer)?;

        let x = ((buffer[0] as i16) << 8) | buffer[1] as i16;
        let y = ((buffer[2] as i16) << 8) | buffer[3] as i16;
        let z = ((buffer[4] as i16) << 8) | buffer[5] as i16;

        Ok([x, y, z])
    }

    /// Reads raw temperature data as 16-bit integer.
    ///
    /// Returns raw sensor values before conversion to Celsius.
    /// Use `read_temperature_celsius()` for temperature in Celsius.
    ///
    /// # Errors
    ///
    /// Returns `Error::I2c` if I2C communication fails.
    pub fn read_temp_raw(&mut self) -> Result<i16, Error<E>> {
        let mut buffer = [0u8; 2];
        self.i2c.write(self.address, &[TEMP_OUT_H])?;
        self.i2c.read(self.address, &mut buffer)?;

        let temp = ((buffer[0] as i16) << 8) | buffer[1] as i16;
        Ok(temp)
    }

    /// Reads processed acceleration data in g-force units.
    ///
    /// # Returns
    ///
    /// `[x, y, z]` array of acceleration values in g-force units
    ///
    /// # Errors
    ///
    /// Returns `Error::I2c` if I2C communication fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use hayasen::mpu9250::Mpu9250;
    /// # let mut mpu = Mpu9250::new(i2c, 0x68);
    /// let acceleration = mpu.read_acceleration()?;
    /// println!("X: {:.2}g, Y: {:.2}g, Z: {:.2}g", 
    ///          acceleration[0], acceleration[1], acceleration[2]);
    /// ```
    pub fn read_acceleration(&mut self) -> Result<[f32; 3], Error<E>> {
        let raw = self.read_accel_raw()?;
        let x = raw[0] as f32 * self.accel_scale;
        let y = raw[1] as f32 * self.accel_scale;
        let z = raw[2] as f32 * self.accel_scale;
        Ok([x, y, z])
    }

    /// Reads processed angular velocity data in degrees per second.
    ///
    /// # Returns
    ///
    /// `[x, y, z]` array of angular velocity values in degrees per second
    ///
    /// # Errors
    ///
    /// Returns `Error::I2c` if I2C communication fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use hayasen::mpu9250::Mpu9250;
    /// # let mut mpu = Mpu9250::new(i2c, 0x68);
    /// let gyro = mpu.read_angular_velocity()?;
    /// println!("X: {:.2}°/s, Y: {:.2}°/s, Z: {:.2}°/s",
    ///          gyro[0], gyro[1], gyro[2]);
    /// ```
    pub fn read_angular_velocity(&mut self) -> Result<[f32; 3], Error<E>> {
        let raw = self.read_gyro_raw()?;
        let x = raw[0] as f32 * self.gyro_scale;  // Fixed: use gyro_scale, not accel_scale
        let y = raw[1] as f32 * self.gyro_scale;
        let z = raw[2] as f32 * self.gyro_scale;
        Ok([x, y, z])
    }

    /// Reads processed temperature data in Celsius.
    ///
    /// # Returns
    ///
    /// Temperature in degrees Celsius
    ///
    /// # Errors
    ///
    /// Returns `Error::I2c` if I2C communication fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use hayasen::mpu9250::Mpu9250;
    /// # let mut mpu = Mpu9250::new(i2c, 0x68);
    /// let temp = mpu.read_temperature_celsius()?;
    /// println!("Temperature: {:.1}°C", temp);
    /// ```
    pub fn read_temperature_celsius(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_temp_raw()?;
        // MPU9250 temperature conversion formula: Temperature (°C) = (TEMP_OUT / 340) + 36.53
        let temperature = (raw as f32) / 340.0 + 36.53;
        Ok(temperature)
    }

    /// Configures the sensor's sample rate divider.
    ///
    /// # Arguments
    ///
    /// * `divider` - Sample rate divider value (0-255)
    ///
    /// # Errors
    ///
    /// Returns `Error::I2c` if I2C communication fails.
    pub fn set_sample_rate(&mut self, divider: u8) -> Result<(), Error<E>> {
        self.i2c.write(self.address, &[SMPRT_DIV, divider])?;
        Ok(())
    }

    /// Configures the digital low-pass filter settings.
    ///
    /// # Arguments
    ///
    /// * `config` - DLPF configuration (`DlpfConfig`)
    ///
    /// # Errors
    ///
    /// Returns `Error::I2c` if I2C communication fails.
    pub fn set_dlpf_config(&mut self, config: DlpfConfig) -> Result<(), Error<E>> {
        let config_value = match config {
            DlpfConfig::Bandwidth260Hz => 0x00,
            DlpfConfig::Bandwidth184Hz => 0x01,
        };
        self.i2c.write(self.address, &[CONFIG, config_value])?;
        Ok(())
    }

    /// Puts the sensor into low-power sleep mode.
    ///
    /// # Errors
    ///
    /// Returns `Error::I2c` if I2C communication fails.
    pub fn enter_sleep_mode(&mut self) -> Result<(), Error<E>> {
        let mut buffer = [0u8];
        self.i2c.write(self.address, &[PWR_MGMT_1])?;
        self.i2c.read(self.address, &mut buffer)?;

        let new_config = buffer[0] | 0x40; // Set sleep bit (bit 6)
        self.i2c.write(self.address, &[PWR_MGMT_1, new_config])?;
        Ok(())
    }

    /// Wakes the sensor from sleep mode.
    ///
    /// # Errors
    ///
    /// Returns `Error::I2c` if I2C communication fails.
    pub fn wake_up(&mut self) -> Result<(), Error<E>> {
        let mut buffer = [0u8];
        self.i2c.write(self.address, &[PWR_MGMT_1])?;
        self.i2c.read(self.address, &mut buffer)?;

        let new_config = buffer[0] & 0xBF; // Clear sleep bit (bit 6) - fixed mask
        self.i2c.write(self.address, &[PWR_MGMT_1, new_config])?;
        Ok(())
    }
}
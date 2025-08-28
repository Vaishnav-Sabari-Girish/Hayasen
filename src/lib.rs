#![no_std]
#![no_main]

//! Hayasen - Multi-sensor driver library for Embedded Rust
//!
//! A comprehensive driver library for various sensors including MPU9250, with more sensors
//! planned for future releases. Provides unified interfaces and easy-to-use APIs for
//! embedded systems development.
//!
//! # Features
//!
//! - **MPU9250 Support**: 9-axis motion tracking (accelerometer, gyroscope, temperature)
//! - **Unified API**: Consistent interface across different sensor types
//! - **`no_std` Compatible**: Designed for embedded systems without allocator
//! - **Feature-based Compilation**: Only compile the sensors you need
//! - **Comprehensive Error Handling**: Unified error type with detailed error information
//!
//! # Quick Start
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! hayasen = { version = "0.1", features = ["mpu9250"] }
//! ```
//!
//! # Examples
//!
//! ## Basic Usage with MPU9250
//!
//! ```rust,no_run
//! use hayasen::prelude::*;
//! use embedded_hal::i2c::I2c;
//!
//! fn main() -> Result<(), Error<MyI2CError>> {
//!     // Easy initialization with default settings
//!     let mut mpu = HayasenFunctions::create_mpu9250_default(i2c, 0x68)?;
//!
//!     // Quick individual readings
//!     let acceleration = HayasenFunctions::read_accel(&mut mpu)?;
//!     let gyroscope = HayasenFunctions::read_gyro(&mut mpu)?;
//!     let temperature = HayasenFunctions::read_temp_c(&mut mpu)?;
//!
//!     // Or read everything at once for efficiency
//!     let (temp, accel, gyro) = HayasenFunctions::read_all(&mut mpu)?;
//!
//!     println!("Temperature: {:.1}°C", temperature);
//!     println!("Acceleration: {:?} g", acceleration);
//!     println!("Gyroscope: {:?} °/s", gyroscope);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Advanced Usage with Custom Configuration
//!
//! ```rust,no_run
//! use hayasen::prelude::*;
//!
//! # fn main() -> Result<(), Error<()>> {
//! // Manual configuration for fine-grained control
//! let mut mpu = Mpu9250::new(i2c, 0x68);
//! mpu.initialize_sensor(
//!     AccelRange::Range4G,      // Higher range for more dynamic motion
//!     GyroRange::Range500Dps,   // Higher range for faster rotation
//! )?;
//!
//! // Configure additional settings
//! mpu.set_sample_rate(4)?;     // Set sample rate divider
//!
//! let data = mpu.read_acceleration()?;
//! # Ok(())
//! # }
//! ```
//!
//! # Cargo Features
//!
//! - `mpu9250`: Enables MPU9250 9-axis motion sensor support (enabled by default)
//! - More sensors coming soon!
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! This crate is guaranteed to compile on stable Rust 1.60.0 and up.

pub mod error;
pub mod functions;
pub mod mpu9250;

pub use error::Error;
pub use functions::HayasenFunctions;

/// Prelude module for convenient imports.
///
/// Import this module to bring all commonly used items into scope:
///
/// ```rust,no_run
/// use hayasen::prelude::*;
/// ```
pub mod prelude {
    pub use crate::error::Error;
    pub use crate::functions::HayasenFunctions;
    pub use crate::mpu9250::{AccelRange, DlpfConfig, GyroRange, Mpu9250};
    pub use embedded_hal::i2c::I2c;
}

/// Convenience wrapper functions for MPU9250 sensor operations.
///
/// These functions provide a simplified interface for common MPU9250 operations
/// using the function registry pattern.
#[cfg(feature = "mpu9250")]
#[cfg_attr(docsrs, doc(cfg(feature = "mpu9250")))]
impl<I2C, E> HayasenFunctions<I2C, E>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    /// Creates and initializes a new MPU9250 sensor with default settings.
    ///
    /// This is a convenience function that handles sensor creation and
    /// initialization with commonly used default values.
    ///
    /// # Arguments
    ///
    /// * `i2c` - I2C peripheral instance
    /// * `address` - I2C address of the MPU9250 (typically 0x68 or 0x69)
    ///
    /// # Returns
    ///
    /// Returns a fully initialized `Mpu9250` instance ready for data reading.
    ///
    /// # Default Configuration
    ///
    /// - Accelerometer: ±2g range
    /// - Gyroscope: ±250 degrees per second range
    /// - Power: Internal oscillator, awake mode
    ///
    /// # Errors
    ///
    /// Returns `Error<I2C::Error>` if initialization fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use hayasen::HayasenFunctions;
    ///
    /// let mpu = HayasenFunctions::create_mpu9250_default(i2c, 0x68)?;
    /// ```
    pub fn create_mpu9250_default(i2c: I2C, address: u8) -> Result<mpu9250::Mpu9250<I2C>, Error<E>> {
        let mut sensor = mpu9250::Mpu9250::new(i2c, address);
        sensor.initialize_sensor(
            mpu9250::AccelRange::Range2G,
            mpu9250::GyroRange::Range250Dps,
        )?;
        Ok(sensor)
    }

    /// Reads acceleration data from MPU9250 sensor.
    ///
    /// # Arguments
    ///
    /// * `sensor` - Reference to initialized MPU9250 sensor
    ///
    /// # Returns
    ///
    /// `[x, y, z]` array of acceleration values in g-force units
    ///
    /// # Errors
    ///
    /// Returns `Error<I2C::Error>` if I2C communication fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use hayasen::HayasenFunctions;
    ///
    /// # let mut mpu = hayasen::mpu9250::Mpu9250::new(i2c, 0x68);
    /// let acceleration = HayasenFunctions::read_accel(&mut mpu)?;
    /// ```
    pub fn read_accel(sensor: &mut mpu9250::Mpu9250<I2C>) -> Result<[f32; 3], Error<E>> {
        let accel_fun = HayasenFunctions::new();
        (accel_fun.mpu9250.read_acceleration)(sensor)
    }

    /// Reads gyroscope data from MPU9250 sensor.
    ///
    /// # Arguments
    ///
    /// * `sensor` - Reference to initialized MPU9250 sensor
    ///
    /// # Returns
    ///
    /// `[x, y, z]` array of angular velocity values in degrees per second
    ///
    /// # Errors
    ///
    /// Returns `Error<I2C::Error>` if I2C communication fails.
    pub fn read_gyro(sensor: &mut mpu9250::Mpu9250<I2C>) -> Result<[f32; 3], Error<E>> {
        let gyro_fun = HayasenFunctions::new();
        (gyro_fun.mpu9250.read_angular_velocity)(sensor)
    }

    /// Reads temperature data from MPU9250 sensor.
    ///
    /// # Arguments
    ///
    /// * `sensor` - Reference to initialized MPU9250 sensor
    ///
    /// # Returns
    ///
    /// Temperature value in degrees Celsius
    ///
    /// # Errors
    ///
    /// Returns `Error<I2C::Error>` if I2C communication fails.
    pub fn read_temp_c(sensor: &mut mpu9250::Mpu9250<I2C>) -> Result<f32, Error<E>> {
        let temp_fun = HayasenFunctions::new();
        (temp_fun.mpu9250.read_temperature_celsius)(sensor)
    }

    /// Reads all available data from MPU9250 sensor in a single operation.
    ///
    /// This is more efficient than reading each value separately as it
    /// minimizes I2C communication overhead.
    ///
    /// # Arguments
    ///
    /// * `sensor` - Reference to initialized MPU9250 sensor
    ///
    /// # Returns
    ///
    /// Tuple containing `(temperature, acceleration, angular_velocity)`
    /// - `temperature`: f32 in degrees Celsius
    /// - `acceleration`: [f32; 3] in g-force units
    /// - `angular_velocity`: [f32; 3] in degrees per second
    ///
    /// # Errors
    ///
    /// Returns `Error<I2C::Error>` if I2C communication fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use hayasen::HayasenFunctions;
    ///
    /// # let mut mpu = hayasen::mpu9250::Mpu9250::new(i2c, 0x68);
    /// let (temp, accel, gyro) = HayasenFunctions::read_all(&mut mpu)?;
    /// ```
    pub fn read_all(sensor: &mut mpu9250::Mpu9250<I2C>) -> Result<(f32, [f32; 3], [f32; 3]), Error<E>> {
        let all_fun = HayasenFunctions::new();
        let temp = (all_fun.mpu9250.read_temperature_celsius)(sensor)?;
        let accel = (all_fun.mpu9250.read_acceleration)(sensor)?;
        let gyro = (all_fun.mpu9250.read_angular_velocity)(sensor)?;
        Ok((temp, accel, gyro))
    }
}

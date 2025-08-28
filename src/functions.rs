//! Function registry for unified sensor access in the Hayasen library.
//!
//! This module provides a centralized registry of function pointers that allow
//! users to access sensor functionality through a unified interface. This is
//! particularly useful for dynamic sensor management or when working with
//! multiple sensor types through a common API.
//!
//! # Feature Availability
//!
//! Each sensor's functionality is conditionally compiled based on Cargo features.
//! Enable the corresponding feature in your `Cargo.toml` to access specific sensors.

#[cfg(feature = "mpu9250")]
pub mod mpu9250 {
    //! Re-export of the MPU9250 module when the `mpu9250` feature is enabled.
    //!
    //! This allows accessing MPU9250-specific types and functions through
    //! `functions::mpu9250::*` when the feature is enabled.
    pub use crate::mpu9250::*;
}

use embedded_hal::i2c::I2c;
use crate::error::Error;

/// Function pointer registry for MPU9250 sensor operations.
///
/// This struct contains function pointers to all available operations
/// for the MPU9250 sensor. It allows dynamic access to sensor functionality
/// without directly depending on the sensor implementation.
///
/// # Type Parameters
///
/// - `I2C`: Type implementing the `I2c` trait for communication
/// - `E`: Error type returned by the I2C implementation
///
/// # Availability
///
/// This struct is only available when the `mpu9250` feature is enabled.
#[cfg(feature = "mpu9250")]
#[cfg_attr(docsrs, doc(cfg(feature = "mpu9250")))]
pub struct MPU9250Functions<I2C, E> {
    /// Verifies sensor identity by reading the WHO_AM_I register
    pub verify_identity: fn(&mut mpu9250::Mpu9250<I2C>) -> Result<(), Error<E>>,
    /// Configures power management settings of the sensor
    pub configure_power: fn(&mut mpu9250::Mpu9250<I2C>) -> Result<(), Error<E>>,
    /// Sets up accelerometer with specified range and sensitivity
    pub setup_accelerometer: fn(&mut mpu9250::Mpu9250<I2C>, mpu9250::AccelRange) -> Result<(), Error<E>>,
    /// Sets up gyroscope with specified range and sensitivity
    pub setup_gyroscope: fn(&mut mpu9250::Mpu9250<I2C>, mpu9250::GyroRange) -> Result<(), Error<E>>,
    /// Comprehensive sensor initialization routine
    pub initialize_sensor: fn(&mut mpu9250::Mpu9250<I2C>, mpu9250::AccelRange, mpu9250::GyroRange) -> Result<(), Error<E>>,
    /// Reads raw accelerometer data as 16-bit integers
    pub read_accel_raw: fn(&mut mpu9250::Mpu9250<I2C>) -> Result<[i16; 3], Error<E>>,
    /// Reads raw gyroscope data as 16-bit integers
    pub read_gyro_raw: fn(&mut mpu9250::Mpu9250<I2C>) -> Result<[i16; 3], Error<E>>,
    /// Reads raw temperature data as 16-bit integer
    pub read_temp_raw: fn(&mut mpu9250::Mpu9250<I2C>) -> Result<i16, Error<E>>,
    /// Reads processed acceleration data in g-force units
    pub read_acceleration: fn(&mut mpu9250::Mpu9250<I2C>) -> Result<[f32; 3], Error<E>>,
    /// Reads processed angular velocity data in degrees per second
    pub read_angular_velocity: fn(&mut mpu9250::Mpu9250<I2C>) -> Result<[f32; 3], Error<E>>,
    /// Reads processed temperature data in Celsius
    pub read_temperature_celsius: fn(&mut mpu9250::Mpu9250<I2C>) -> Result<f32, Error<E>>,
    /// Configures the sensor's sample rate divider
    pub set_sample_rate: fn(&mut mpu9250::Mpu9250<I2C>, u8) -> Result<(), Error<E>>,
    /// Configures the digital low-pass filter settings
    pub set_dlpf_config: fn(&mut mpu9250::Mpu9250<I2C>, mpu9250::DlpfConfig) -> Result<(), Error<E>>,
    /// Puts the sensor into low-power sleep mode
    pub enter_sleep_mode: fn(&mut mpu9250::Mpu9250<I2C>) -> Result<(), Error<E>>,
    /// Wakes the sensor from sleep mode
    pub wake_up: fn(&mut mpu9250::Mpu9250<I2C>) -> Result<(), Error<E>>,
}

/// Main registry struct holding all available sensor function collections.
///
/// This struct serves as the central access point for all sensor functionality
/// through function pointers. It allows users to access sensor operations
/// without directly coupling to specific sensor implementations.
///
/// # Type Parameters
///
/// - `I2C`: Type implementing the `I2c` trait for communication
/// - `E`: Error type returned by the I2C implementation
///
/// # Examples
///
/// ```rust,no_run
/// use hayasen::functions::HayasenFunctions;
/// use hayasen::mpu9250::Mpu9250;
/// use embedded_hal::i2c::I2c;
///
/// # fn main() -> Result<(), hayasen::Error<()>> {
/// // Create function registry
/// let functions = HayasenFunctions::new();
///
/// // Create sensor instance
/// let mut mpu_sensor = Mpu9250::new(i2c_peripheral, 0x68);
///
/// // Use function pointers through the registry
/// (functions.mpu9250.initialize_sensor)(
///     &mut mpu_sensor,
///     hayasen::mpu9250::AccelRange::Range2G,
///     hayasen::mpu9250::GyroRange::Range250Dps
/// )?;
///
/// let acceleration = (functions.mpu9250.read_acceleration)(&mut mpu_sensor)?;
/// # Ok(())
/// # }
/// ```
pub struct HayasenFunctions<I2C, E> {
    /// MPU9250 sensor function registry
    ///
    /// Available only when the `mpu9250` feature is enabled.
    #[cfg(feature = "mpu9250")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mpu9250")))]
    pub mpu9250: MPU9250Functions<I2C, E>,
}

impl<I2C, E> HayasenFunctions<I2C, E> 
where 
    I2C: I2c<Error = E>
{
    /// Creates a new function registry with all available sensor functions.
    ///
    /// This constructor initializes all function pointers to their corresponding
    /// methods in the sensor implementations. The returned registry provides
    /// access to all sensor operations that are enabled through Cargo features.
    ///
    /// # Returns
    ///
    /// A new `HayasenFunctions` instance with all available function pointers
    /// properly initialized.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use hayasen::functions::HayasenFunctions;
    ///
    /// let sensor_functions = HayasenFunctions::new();
    /// // Now use sensor_functions.mpu9250.* to access MPU9250 functionality
    /// ```
    pub fn new() -> Self {
        HayasenFunctions {
            #[cfg(feature = "mpu9250")]
            mpu9250: MPU9250Functions {
                verify_identity: mpu9250::Mpu9250::verify_identity,
                configure_power: mpu9250::Mpu9250::configure_power,
                setup_accelerometer: mpu9250::Mpu9250::setup_accelerometer,
                setup_gyroscope: mpu9250::Mpu9250::setup_gyroscope,
                initialize_sensor: mpu9250::Mpu9250::initialize_sensor,
                read_accel_raw: mpu9250::Mpu9250::read_accel_raw,
                read_gyro_raw: mpu9250::Mpu9250::read_gyro_raw,
                read_temp_raw: mpu9250::Mpu9250::read_temp_raw,
                read_acceleration: mpu9250::Mpu9250::read_acceleration,
                read_angular_velocity: mpu9250::Mpu9250::read_angular_velocity,
                read_temperature_celsius: mpu9250::Mpu9250::read_temperature_celsius,
                set_sample_rate: mpu9250::Mpu9250::set_sample_rate,
                set_dlpf_config: mpu9250::Mpu9250::set_dlpf_config,
                enter_sleep_mode: mpu9250::Mpu9250::enter_sleep_mode,
                wake_up: mpu9250::Mpu9250::wake_up,
            }
        }
    }
}

// Implement Default for convenience
impl<I2C, E> Default for HayasenFunctions<I2C, E>
where
    I2C: I2c<Error = E>,
{
    /// Provides a default instance of the function registry.
    ///
    /// This is equivalent to calling `HayasenFunctions::new()`.
    fn default() -> Self {
        Self::new()
    }
}
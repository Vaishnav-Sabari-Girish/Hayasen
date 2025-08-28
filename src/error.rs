//! Error types for the Hayasen sensor driver library.
//!
//! This module defines the unified error type [`Error`] used throughout the library
//! to handle all possible failure scenarios when working with sensors.

use core::fmt::{Debug, Formatter, Result};

/// Unified error type for all sensor operations in the Hayasen library.
///
/// This enum encapsulates various error types that can occur when communicating with
/// or configuring sensors. It supports generic I2C errors while providing
/// sensor-specific error variants.
///
/// # Examples
///
/// ```rust
/// use hayasen::error::Error;
///
/// // Creating different error types
/// let i2c_error = Error::I2c(std::io::Error::new(std::io::ErrorKind::Other, "I2C bus busy"));
/// let config_error = Error::ConfigError;
/// let sensor_error = Error::SensorSpecific("Sensor calibration failed");
/// ```
#[derive(Clone, PartialEq, Eq)]
pub enum Error<E> {
    /// Error originating from the I2C bus communication.
    ///
    /// This variant wraps the underlying I2C peripheral's error type,
    /// allowing library users to handle hardware-specific I2C errors.
    I2c(E),
    
    /// Invalid or malformed data received from the sensor.
    ///
    /// This typically indicates:
    /// - Checksum validation failures
    /// - Data outside expected ranges
    /// - Malformed sensor responses
    InvalidData,
    
    /// Sensor not detected at the expected I2C address.
    ///
    /// This error occurs when:
    /// - Sensor is not connected or powered
    /// - Wrong I2C address specified
    /// - Sensor is in sleep mode and not responding
    NotDetected,
    
    /// Invalid configuration parameter provided to the sensor.
    ///
    /// This includes:
    /// - Out-of-range values for sensor settings
    /// - Invalid combination of configuration flags
    /// - Unsupported operating modes
    ConfigError,
    
    /// Sensor-specific error with a descriptive message.
    ///
    /// This variant provides additional context for errors that are
    /// specific to particular sensor models or functionalities.
    SensorSpecific(&'static str),
}

impl<E> From<E> for Error<E> {
    /// Converts an underlying I2C error into a Hayasen [`Error`].
    ///
    /// This implementation allows using the `?` operator with I2C operations
    /// that return their native error type, automatically converting them
    /// to [`Error::I2c`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hayasen::error::Error;
    /// use std::io::{Error, ErrorKind};
    ///
    /// let io_error = Error::new(ErrorKind::Other, "I2C timeout");
    /// let hayasen_error: Error<Error> = io_error.into();
    /// ```
    fn from(error: E) -> Self {
        Error::I2c(error)
    }
}

impl<E> Debug for Error<E> 
where 
    E: Debug 
{
    /// Formats the error for display and debugging purposes.
    ///
    /// Provides human-readable descriptions of each error variant,
    /// including the underlying I2C error details when available.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hayasen::error::Error;
    /// use std::fmt::{Debug, Formatter, Result};
    ///
    /// let error = Error::<std::io::Error>::NotDetected;
    /// println!("Error: {:?}", error); // Prints: "Sensor not detected at address"
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Error::I2c(e) => write!(f, "I2C Error: {:?}", e),
            Error::InvalidData => write!(f, "Invalid Data received from sensor"),
            Error::NotDetected => write!(f, "Sensor not detected at address"),
            Error::ConfigError => write!(f, "Invalid Configuration"),
            Error::SensorSpecific(msg) => write!(f, "Sensor Error: {}", msg),
        }
    }
}

// Additional convenience implementations
impl<E> Error<E> {
    /// Returns `true` if the error is an I2C communication error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hayasen::error::Error;
    ///
    /// let i2c_error = Error::I2c("bus error");
    /// assert!(i2c_error.is_i2c_error());
    ///
    /// let other_error = Error::NotDetected;
    /// assert!(!other_error.is_i2c_error());
    /// ```
    pub fn is_i2c_error(&self) -> bool {
        matches!(self, Error::I2c(_))
    }
    
    /// Returns `true` if the error is related to sensor configuration.
    ///
    /// This includes both [`Error::ConfigError`] and sensor-specific errors
    /// that might indicate configuration issues.
    pub fn is_config_error(&self) -> bool {
        matches!(self, Error::ConfigError | Error::SensorSpecific(_))
    }
    
    /// Converts the error into its underlying I2C error if it is one.
    ///
    /// Returns `Some(E)` if this is an [`Error::I2c`], otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hayasen::error::Error;
    ///
    /// let i2c_error = Error::I2c("bus error");
    /// assert_eq!(i2c_error.into_i2c_error(), Some("bus error"));
    ///
    /// let other_error = Error::NotDetected;
    /// assert_eq!(other_error.into_i2c_error(), None);
    /// ```
    pub fn into_i2c_error(self) -> Option<E> {
        match self {
            Error::I2c(e) => Some(e),
            _ => None,
        }
    }
}
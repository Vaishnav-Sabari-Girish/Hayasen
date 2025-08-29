# Errors

## Overview

The `Error` enum provides a unified mechanism for handling errors across the entire crate, covering both
low-level I²C communication issues and higher-level sensor logic problems. It is designed to be reusable
across multiple sensor drivers (e.g., `mpu9250.rs`) and is compatible with functions in `functions.rs` and
`lib.rs` that propagate errors via `Result`.

### Purpose

* To represent all possible error conditions in a consistent way.
* To integrate seamlessly with external I²C error types via `From<E>`.
* To simplify debugging and error inspection across the crate.

---

## Enum: `Error<E>`

### Definition

```rust
pub enum Error<E> {
    I2c(E),
    InvalidData,
    NotDetected,
    ConfigError,
    SensorSpecific(&'static str),
}
```

### Variants

* **`I2c(E)`**

  * Wraps the underlying I²C communication error of type `E`.
  * Enables automatic propagation of hardware-level errors from sensor drivers.

* **`InvalidData`**

  * Indicates data received from the sensor could not be parsed or was corrupted.

* **`NotDetected`**

  * Raised when a sensor is not found at its expected address or fails to acknowledge.

* **`ConfigError`**

  * Represents an invalid configuration parameter or a failed setup process.

* **`SensorSpecific(&'static str)`**

  * Used for driver-specific errors not covered by other variants.
  * Stores a descriptive static string message.

---

## Trait Implementations

### `From<E> for Error<E>`

This implementation allows automatic conversion from I²C-specific errors into the crate-wide `Error` type:

```rust
impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::I2c(error)
    }
}
```

This enables ergonomic use of `?` in functions returning `Result<T, Error<E>>`.

### `Debug` Implementation

Provides human-readable descriptions of errors, aiding debugging during development and testing.

---

## Convenience Methods

* **`is_i2c_error(&self) -> bool`** – Checks if the error originated from I²C communication.
* **`is_config_error(&self) -> bool`** – Returns `true` if the error relates to configuration or sensor-specific settings.
* **`into_i2c_error(self) -> Option<E>`** – Consumes the error and extracts the original I²C error if present.

---

## Usage in the Crate

* In `mpu9250.rs`, sensor initialization and data reading functions propagate `Error<E>` to signal issues with configuration, detection, or raw data.
* In `functions.rs`, helper routines use `Error` to unify return types across different hardware abstraction layers.
* In `lib.rs`, `Error` is exposed as part of the crate API, making it available to external applications.

---

## Example Usage

```rust
fn init_sensor<I2C, E>(i2c: &mut I2C) -> Result<(), Error<E>>
where
    I2C: I2c<Error = E>,
    E: Debug,
{
    i2c.write(0x68, &[0x00]).map_err(Error::from)?;
    Ok(())
}
```

This function returns `Error<E>` for I²C failures or sensor-specific issues, integrating seamlessly with
the rest of the crate.

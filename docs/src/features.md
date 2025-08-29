# Features of Hayasen

## Overview

Hayasen is designed to be a lightweight, modular sensor driver crate with a focus on embedded systems. It aims to provide a unified interface for interacting with various sensors, starting with I²C-based devices and expanding to support other communication protocols in future releases.

---

## Key Features

### 1. **Unified Error Handling**

* Centralized `Error` enum for consistent error reporting across all drivers.
* Distinguishes between:

  * Low-level I²C errors
  * Data integrity issues
  * Configuration and initialization failures
  * Sensor-specific errors
* Convenience methods (`is_i2c_error`, `is_config_error`, `into_i2c_error`) for error inspection and recovery.

### 2. **MPU9250 Sensor Support**

* Initial implementation supports the MPU9250 IMU sensor.
* Provides methods for:

  * Initialization and configuration of accelerometer, gyroscope, and magnetometer.
  * Reading raw and processed data from all sensor units.
  * Power management (e.g., wake/sleep modes).
* Modular driver structure allows easy adaptation to similar IMU devices.

### 3. **Modular Design for Sensor Expansion**

* Core library is structured to support additional sensors without rewriting core logic.
* Shared abstractions for configuration, data reading, and error handling.
* Planned future support for:

  * SPI-based sensors
  * Analog sensors (via ADC interfaces)
  * Digital sensors beyond I²C.

### 4. **Lightweight and `no_std` Compatible**

* Designed for embedded environments with constrained resources.
* Avoids allocations and unnecessary dependencies.
* Compatible with `no_std` for use in bare-metal microcontrollers.

### 5. **Extensible Configuration System**

* Supports runtime configuration of sensor parameters.
* Designed to allow sensor-specific tuning (e.g., sensitivity ranges, filter settings).
* Provides error reporting for invalid configurations via `Error::ConfigError`.

### 6. **Developer-Friendly API**

* Intuitive function naming and structured module layout.
* Encourages readable, idiomatic Rust code.
* Provides clear debugging output via `Debug` trait implementations.

---

## Future Features (Planned)

* Support for additional sensors (environmental, proximity, etc.).
* Integration with `embedded-hal` traits for seamless compatibility with Rust embedded ecosystem.
* Optional calibration utilities and sensor fusion algorithms.
* Benchmarking and performance profiling tools for real-time applications.

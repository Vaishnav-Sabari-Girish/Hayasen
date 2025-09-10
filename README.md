<p align="center">
  <img src="./hayasen_logo.png" alt="Sublime's custom image"/>
</p>

# Hayasen

[![Crates.io](https://img.shields.io/crates/v/hayasen.svg)](https://crates.io/crates/hayasen)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-APACHE)
[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.60.0-blue.svg)](https://rust-lang.org)

> ⚠️ **Warning: Early Development Stage**
> 
> This library is currently under active development and has not been thoroughly tested.
> Use in production systems at your own risk. APIs may change without notice.

Hayasen is an Embedded Rust library designed to simplify sensor integration in embedded systems. 
It provides unified, type-safe interfaces for various sensors with a focus on ease of use and reliability.

## 🚀 Currently Supported Sensors

- **MPU9250** - 9-axis Inertial Measurement Unit (accelerometer, gyroscope, temperature, magnetometer) - *Work in Progress*
- **MPU6050** - 6-axis Inertial Measurement Unit (accelerometer, gyroscope, temperature)


## Progress 

Click here to check the current progress of the library

[Progress](./progress.md)

## 📦 Installation

Add `hayasen` to your `Cargo.toml`:

```toml
[dependencies]
hayasen = { version = "*", features = ["mpu9250"] }   # For MPU9250 support
```

## 🎯 Quick Start

```rust
use hayasen::prelude::*;
use hayasen::mpu9250_hayasen;

fn main() -> Result<(), Error<YourI2cError>> {
    // Setup I2C (platform-specific)
    let i2c = setup_i2c();
    
    // Initialize sensor with default configuration
    let mut sensor = mpu9250_hayasen::create_default(i2c, 0x68)?;
    
    // Read all sensor data (temp, accel, gyro)
    let (temperature, acceleration, angular_velocity) =
        mpu9250_hayasen::read_all(&mut sensor)?;
    
    println!("Temperature: {:.2}°C", temperature);
    println!("Acceleration: [{:.3}, {:.3}, {:.3}] g",
             acceleration[0], acceleration[1], acceleration[2]);
    println!("Angular Velocity: [{:.3}, {:.3}, {:.3}] dps",
             angular_velocity[0], angular_velocity[1], angular_velocity[2]);
    
    Ok(())
}
```

## 🔧 Features

- `mpu9250` - Enables MPU9250 Inertial Measurement Unit support (enabled by default)
- `mpu6050` - Enables MPU6050 Inertial Measurement Unit support 
- More sensors coming soon!

## 📚 Documentation

- [API Documentation](https://vaishnav.world/Hayasen) - Complete API reference
- [Examples](https://github.com/Vaishnav-Sabari-Girish/Hayasen-Examples) - Practical usage examples   (Coming Soon)
- [Contributing Guidelines](./CONTRIBUTING.md) - How to contribute to the project

## 🏗️ Project Structure

```
hayasen/
├── src/
│   ├── lib.rs          # Main library entry point
│   ├── error.rs        # Unified error types
│   ├── functions.rs    # Function registry system
│   ├── mpu9250.rs      # MPU9250 sensor implementation
│   └── mpu6050.rs      # MPU6050 sensor implementation
├── examples/           # Usage examples
└── tests/              # Integration tests
```

## 📝 License

This project is dual-licensed under either:
- Apache License, Version 2.0 [Apache-2.0](LICENSE-APACHE)
- MIT license [MIT](LICENSE-MIT)

## 🙏 Acknowledgments
- Inspired by various embedded Rust driver libraries
- Thanks to the Rust embedded working group for excellent tools and guidance
- Community contributors and testers

## 🐛 Issue Reporting

Found a bug or have a feature request? Please open an [issue](https://github.com/Vaishnav-Sabari-Girish/Hayasen/issues) on GitHub.


## 🔮 Roadmap

### 🎯 Short-term Goals (v0.2.0 - v0.5.0)

- [ ] **Complete MPU9250 implementation and testing**
  - [ ] Magnetometer support (AK8963)
  - [ ] Self-test functionality
  - [ ] Motion detection interrupts
  - [ ] FIFO buffer support
  - [ ] Comprehensive unit tests
  - [ ] Integration tests with hardware

- [ ] **Add comprehensive test suite**
  - [ ] Unit tests for all public APIs
  - [ ] Integration tests with mock I2C
  - [ ] Hardware-in-the-loop testing
  - [ ] Continuous integration setup
  - [ ] Code coverage reporting

- [ ] **Create more usage examples**
  - [ ] Basic sensor reading example
  - [ ] Interrupt-driven motion detection
  - [ ] Data logging application
  - [ ] Multi-sensor fusion example
  - [ ] RTOS integration examples

### 🚀 Medium-term Goals (v0.6.0 - v0.9.0)

- [ ] **Add MAX30102 heart rate sensor support**
  - [ ] Heart rate monitoring
  - [ ] Oxygen saturation (SpO2) measurement
  - [ ] FIFO data reading
  - [ ] Interrupt configuration
  - [ ] Temperature reading

- [ ] **Add CI/CD pipeline**
  - [ ] GitHub Actions for testing
  - [ ] Automated documentation deployment
  - [ ] Release automation
  - [ ] Crate publishing automation
  - [ ] Cross-compilation testing

### 🌟 Long-term Goals (v1.0.0+)

- [ ] **Support for more sensor types**
  - [ ] BME280 (Temperature, Humidity, Pressure)
  - [ ] BMP180/BMP280 (Pressure)
  - [ ] HMC5883L (Magnetometer)
  - [ ] ADXL345 (Accelerometer)
  - [ ] TMP36/TMP102 (Temperature)

- [ ] **Advanced features**
  - [ ] Sensor fusion algorithms
  - [ ] Power management utilities
  - [ ] Async/await support
  - [ ] No-alloc mode for tiny systems
  - [ ] WebAssembly support for simulation

- [ ] **Ecosystem integration**
  - [ ] Embassy framework support
  - [ ] RTIC framework integration
  - [ ] defmt logging support
  - [ ] Probe-rs debugging support
  - [ ] Platform-agnostic drivers

### 📊 Version Timeline

```mermaid
timeline
    title Hayasen Development Timeline
    section v0.2.x
        MPU9250 Completion : Magnetometer<br>Interrupts<br>FIFO
        Testing Suite      : Unit Tests<br>Integration Tests
    section v0.3.x
        Examples           : Basic Examples<br>Advanced Use Cases
        CI/CD              : GitHub Actions<br>Auto Documentation
    section v0.4.x
        MAX30102 Support   : Heart Rate<br>SpO2 Monitoring
    section v0.5.x
        Additional Sensors : BME280<br>HMC5883L
    section v1.0.0
        Stable API         : Production Ready<br>Full Documentation
```


## 📊 Project Activity

### Stargazers over time

[![Stargazers over time](https://starchart.cc/Vaishnav-Sabari-Girish/Hayasen.svg?variant=adaptive)](https://starchart.cc/Vaishnav-Sabari-Girish/Hayasen)

### Stargazers

[![Stargazers repo roster for @Vaishnav-Sabari-Girish/Hayasen](https://reporoster.com/stars/dark/Vaishnav-Sabari-Girish/Hayasen)](https://github.com/Vaishnav-Sabari-Girish/Hayasen/stargazers)

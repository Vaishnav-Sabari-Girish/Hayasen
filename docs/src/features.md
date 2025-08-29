# Features

This section details the key features and capabilities provided by the `hayasen` library, along with
future development plans.

## Current Features

- `mpu9250` - Enables comprehensive support for the MPU9250 motion sensor. This includes functionalities
for reading accelerometer, gyroscope, and magnetometer data, as well as configuration options for various
operating modes. (Enabled by default)

## Future Plans

### 🔮 Roadmap

#### 🎯 Short-term Goals (v0.2.0 - v0.5.0)

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

#### 🚀 Medium-term Goals (v0.6.0 - v0.9.0)

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

#### 🌟 Long-term Goals (v1.0.0+)

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

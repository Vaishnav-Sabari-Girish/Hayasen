# MPU9250 Usage Guide

## Overview

The Hayasen MPU9250 library provides a comprehensive interface for working with the MPU9250 9-axis motion tracking device. This guide demonstrates how to use the library in various scenarios, from basic sensor reading to advanced configuration.

```admonish note title="Please Note", collapsible=false
Please note that the I2C instance used here currently is of datatype `TWIM` which corresponds to the I2C instance of a nRF Nordic microcontroller.
So basically `hayasen` currently only supports Nordic Microcontrollers
```

## Basic Usage

### Quick Start with Default Configuration

The simplest way to get started is using the `create_default` function from the `mpu9250_hayasen` module:

```rust
use hayasen::prelude::*;
use hayasen::mpu9250_hayasen;

fn main() -> Result<(), Error<YourI2cError>> {
    // Assume you have an I2C peripheral instance
    let i2c = setup_i2c(); // Your platform-specific I2C setup
    let mpu_address = 0x68; // Default MPU9250 I2C address
    
    // Create sensor with default configuration (2G accel, 250 DPS gyro)
    let mut sensor = mpu9250_hayasen::create_default(i2c, mpu_address)?;
    
    // Read all sensor data
    let (temperature, acceleration, angular_velocity) = mpu9250_hayasen::read_all(&mut sensor)?;
    
    println!("Temperature: {:.2}°C", temperature);
    println!("Acceleration [X, Y, Z]: [{:.3}, {:.3}, {:.3}] g", 
             acceleration[0], acceleration[1], acceleration[2]);
    println!("Angular Velocity [X, Y, Z]: [{:.3}, {:.3}, {:.3}] dps", 
             angular_velocity[0], angular_velocity[1], angular_velocity[2]);
    
    Ok(())
}
```

### Individual Sensor Readings

You can read each sensor independently:

```rust
use hayasen::prelude::*;
use hayasen::mpu9250_hayasen;

fn read_individual_sensors() -> Result<(), Error<YourI2cError>> {
    let i2c = setup_i2c();
    let mut sensor = mpu9250_hayasen::create_default(i2c, 0x68)?;
    
    // Read acceleration only
    let accel = mpu9250_hayasen::read_acceleration(&mut sensor)?;
    println!("Acceleration: X={:.3}g, Y={:.3}g, Z={:.3}g", accel[0], accel[1], accel[2]);
    
    // Read gyroscope only
    let gyro = mpu9250_hayasen::read_angular_velocity(&mut sensor)?;
    println!("Angular Velocity: X={:.3}°/s, Y={:.3}°/s, Z={:.3}°/s", gyro[0], gyro[1], gyro[2]);
    
    // Read temperature only
    let temp = mpu9250_hayasen::read_temperature(&mut sensor)?;
    println!("Temperature: {:.2}°C", temp);
    
    Ok(())
}
```

## Advanced Configuration

### Manual Sensor Setup

For more control over sensor configuration, use the direct API:

```rust
use hayasen::prelude::*;

fn advanced_setup() -> Result<(), Error<YourI2cError>> {
    let i2c = setup_i2c();
    let mut sensor = Mpu9250::new(i2c, 0x68);
    
    // Manual initialization with custom ranges
    sensor.initialize_sensor(
        AccelRange::Range8G,      // Higher acceleration range
        GyroRange::Range1000Dps   // Higher angular velocity range
    )?;
    
    // Configure sample rate (divider from 1kHz base rate)
    // Sample rate = 1000Hz / (1 + divider)
    sensor.set_sample_rate(9)?; // 100Hz sample rate
    
    // Configure digital low-pass filter
    sensor.set_dlpf_config(DlpfConfig::Bandwidth184Hz)?;
    
    Ok(())
}
```

### Reading Raw Data

For applications requiring raw ADC values:

```rust
use hayasen::prelude::*;

fn read_raw_data() -> Result<(), Error<YourI2cError>> {
    let i2c = setup_i2c();
    let mut sensor = Mpu9250::new(i2c, 0x68);
    sensor.initialize_sensor(AccelRange::Range2G, GyroRange::Range250Dps)?;
    
    // Read raw 16-bit values
    let raw_accel = sensor.read_accel_raw()?;
    let raw_gyro = sensor.read_gyro_raw()?;
    let raw_temp = sensor.read_temp_raw()?;
    
    println!("Raw Accelerometer: X={}, Y={}, Z={}", raw_accel[0], raw_accel[1], raw_accel[2]);
    println!("Raw Gyroscope: X={}, Y={}, Z={}", raw_gyro[0], raw_gyro[1], raw_gyro[2]);
    println!("Raw Temperature: {}", raw_temp);
    
    Ok(())
}
```

## Power Management

### Sleep Mode Operation

```rust
use hayasen::prelude::*;

fn power_management_example() -> Result<(), Error<YourI2cError>> {
    let i2c = setup_i2c();
    let mut sensor = Mpu9250::new(i2c, 0x68);
    sensor.initialize_sensor(AccelRange::Range2G, GyroRange::Range250Dps)?;
    
    // Normal operation
    let data = sensor.read_acceleration()?;
    println!("Before sleep: {:?}", data);
    
    // Enter sleep mode to save power
    sensor.enter_sleep_mode()?;
    println!("Sensor in sleep mode");
    
    // Wake up and resume operation
    sensor.wake_up()?;
    
    // Read data after waking up
    let data_after_wake = sensor.read_acceleration()?;
    println!("After wake: {:?}", data_after_wake);
    
    Ok(())
}
```

## Real-World Applications

### Motion Detection Example

```rust
use hayasen::prelude::*;
use hayasen::mpu9250_hayasen;

fn motion_detection_loop() -> Result<(), Error<YourI2cError>> {
    let i2c = setup_i2c();
    let mut sensor = mpu9250_hayasen::create_default(i2c, 0x68)?;
    
    // Motion detection thresholds
    const ACCEL_THRESHOLD: f32 = 0.1; // g
    const GYRO_THRESHOLD: f32 = 5.0;  // degrees per second
    
    loop {
        let (temp, accel, gyro) = mpu9250_hayasen::read_all(&mut sensor)?;
        
        // Calculate total acceleration magnitude (subtract gravity)
        let total_accel = (accel[0].powi(2) + accel[1].powi(2) + accel[2].powi(2)).sqrt();
        let motion_accel = (total_accel - 1.0).abs(); // Subtract 1g gravity
        
        // Calculate total angular velocity
        let total_gyro = (gyro[0].powi(2) + gyro[1].powi(2) + gyro[2].powi(2)).sqrt();
        
        // Detect motion
        if motion_accel > ACCEL_THRESHOLD || total_gyro > GYRO_THRESHOLD {
            println!("Motion detected! Accel: {:.3}g, Gyro: {:.3}°/s", motion_accel, total_gyro);
        }
        
        // Small delay between readings
        delay_ms(50);
    }
}
```

### Data Logging Example

```rust
use hayasen::prelude::*;

fn data_logging_example() -> Result<(), Error<YourI2cError>> {
    let i2c = setup_i2c();
    let mut sensor = Mpu9250::new(i2c, 0x68);
    
    // Configure for high-precision data logging
    sensor.initialize_sensor(AccelRange::Range4G, GyroRange::Range500Dps)?;
    sensor.set_sample_rate(19)?; // 50Hz sampling
    sensor.set_dlpf_config(DlpfConfig::Bandwidth184Hz)?;
    
    let mut sample_count = 0;
    const MAX_SAMPLES: usize = 1000;
    
    while sample_count < MAX_SAMPLES {
        let timestamp = get_timestamp(); // Your platform-specific timestamp
        let accel = sensor.read_acceleration()?;
        let gyro = sensor.read_angular_velocity()?;
        let temp = sensor.read_temperature_celsius()?;
        
        // Log data (implement your own logging mechanism)
        log_data(timestamp, accel, gyro, temp);
        
        sample_count += 1;
        delay_ms(20); // 50Hz sampling
    }
    
    Ok(())
}
```

## Configuration Options

### Accelerometer Ranges

```rust
use hayasen::prelude::*;

// Available accelerometer ranges and their use cases
fn configure_accelerometer_ranges() -> Result<(), Error<YourI2cError>> {
    let i2c = setup_i2c();
    let mut sensor = Mpu9250::new(i2c, 0x68);
    
    // Choose range based on application:
    
    // For precise, low-acceleration measurements (e.g., tilt sensing)
    sensor.setup_accelerometer(AccelRange::Range2G)?;
    
    // For general motion detection
    sensor.setup_accelerometer(AccelRange::Range4G)?;
    
    // For high-impact applications (e.g., crash detection)
    sensor.setup_accelerometer(AccelRange::Range8G)?;
    
    // For extreme acceleration measurements
    sensor.setup_accelerometer(AccelRange::Range16G)?;
    
    Ok(())
}
```

### Gyroscope Ranges

```rust
use hayasen::prelude::*;

fn configure_gyroscope_ranges() -> Result<(), Error<YourI2cError>> {
    let i2c = setup_i2c();
    let mut sensor = Mpu9250::new(i2c, 0x68);
    
    // Choose range based on expected rotation rates:
    
    // For slow, precise rotations (e.g., stabilization)
    sensor.setup_gyroscope(GyroRange::Range250Dps)?;
    
    // For moderate rotation rates (e.g., drone control)
    sensor.setup_gyroscope(GyroRange::Range500Dps)?;
    
    // For fast rotations (e.g., sports analysis)
    sensor.setup_gyroscope(GyroRange::Range1000Dps)?;
    
    // For very high rotation rates (e.g., spinning objects)
    sensor.setup_gyroscope(GyroRange::Range2000Dps)?;
    
    Ok(())
}
```

## Error Handling

### Comprehensive Error Handling

```rust
use hayasen::prelude::*;
use hayasen::mpu9250_hayasen;

fn robust_sensor_operation() {
    let i2c = setup_i2c();
    
    match mpu9250_hayasen::create_default(i2c, 0x68) {
        Ok(mut sensor) => {
            loop {
                match mpu9250_hayasen::read_all(&mut sensor) {
                    Ok((temp, accel, gyro)) => {
                        process_sensor_data(temp, accel, gyro);
                    },
                    Err(e) => {
                        match e {
                            Error::I2c(_) => {
                                println!("I2C communication error, retrying...");
                                delay_ms(100);
                                continue;
                            },
                            Error::NotDetected => {
                                println!("Sensor not detected, check wiring");
                                break;
                            },
                            Error::InvalidData => {
                                println!("Invalid data received, skipping reading");
                                continue;
                            },
                            Error::ConfigError => {
                                println!("Configuration error");
                                break;
                            },
                            Error::SensorSpecific(msg) => {
                                println!("Sensor-specific error: {}", msg);
                                break;
                            },
                        }
                    }
                }
                delay_ms(20);
            }
        },
        Err(e) => {
            println!("Failed to initialize sensor: {:?}", e);
        }
    }
}
```

### Error Classification

```rust
use hayasen::prelude::*;

fn classify_errors(error: Error<YourI2cError>) {
    if error.is_i2c_error() {
        println!("Communication problem - check wiring and I2C bus");
    } else if error.is_config_error() {
        println!("Configuration issue - check sensor settings");
    }
    
    // Extract underlying I2C error if needed
    if let Some(i2c_err) = error.into_i2c_error() {
        handle_i2c_specific_error(i2c_err);
    }
}
```

## Platform-Specific Examples

### ESP32 Example (using esp-hal)

```rust
#![no_std]
#![no_main]

use esp_hal::{
    clock::ClockControl,
    i2c::I2C,
    peripherals::Peripherals,
    prelude::*,
    delay::Delay,
};
use hayasen::prelude::*;
use hayasen::mpu9250_hayasen;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    
    // Setup I2C
    let i2c = I2C::new(
        peripherals.I2C0,
        peripherals.GPIO21, // SDA
        peripherals.GPIO22, // SCL
        100u32.kHz(),
        &clocks,
    );
    
    let mut delay = Delay::new(&clocks);
    
    // Initialize MPU9250
    let mut sensor = match mpu9250_hayasen::create_default(i2c, 0x68) {
        Ok(s) => s,
        Err(_) => {
            println!("Failed to initialize MPU9250");
            loop { delay.delay_ms(1000u32); }
        }
    };
    
    loop {
        match mpu9250_hayasen::read_all(&mut sensor) {
            Ok((temp, accel, gyro)) => {
                println!("T: {:.1}°C | A: [{:.2}, {:.2}, {:.2}]g | G: [{:.1}, {:.1}, {:.1}]°/s",
                        temp, accel[0], accel[1], accel[2], gyro[0], gyro[1], gyro[2]);
            },
            Err(_) => println!("Read error"),
        }
        delay.delay_ms(100u32);
    }
}
```

### STM32 Example (using stm32f4xx-hal)

```rust
#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32f4xx_hal::{
    pac,
    prelude::*,
    i2c::I2c,
};
use hayasen::prelude::*;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();
    
    let gpiob = dp.GPIOB.split();
    let scl = gpiob.pb8.into_alternate_open_drain();
    let sda = gpiob.pb9.into_alternate_open_drain();
    
    let i2c = I2c::new(dp.I2C1, (scl, sda), 400.kHz(), &clocks);
    
    let mut delay = cortex_m::delay::Delay::new(cp.SYST, clocks.hclk().to_Hz());
    let mut sensor = Mpu9250::new(i2c, 0x68);
    
    // Custom initialization
    match sensor.initialize_sensor(AccelRange::Range4G, GyroRange::Range500Dps) {
        Ok(_) => {},
        Err(_) => loop { delay.delay_ms(1000u32); }
    }
    
    loop {
        if let Ok(accel) = sensor.read_acceleration() {
            // Process acceleration data
            process_motion_data(accel);
        }
        delay.delay_ms(50u32);
    }
}
```

## Advanced Use Cases

### Calibration and Offset Correction

```rust
use hayasen::prelude::*;

fn calibrate_sensor() -> Result<[f32; 6], Error<YourI2cError>> {
    let i2c = setup_i2c();
    let mut sensor = Mpu9250::new(i2c, 0x68);
    sensor.initialize_sensor(AccelRange::Range2G, GyroRange::Range250Dps)?;
    
    println!("Keep sensor stationary for calibration...");
    delay_ms(2000);
    
    let mut accel_offsets = [0.0f32; 3];
    let mut gyro_offsets = [0.0f32; 3];
    const SAMPLES: usize = 100;
    
    // Collect calibration samples
    for _ in 0..SAMPLES {
        let accel = sensor.read_acceleration()?;
        let gyro = sensor.read_angular_velocity()?;
        
        accel_offsets[0] += accel[0];
        accel_offsets[1] += accel[1];
        accel_offsets[2] += accel[2] - 1.0; // Subtract expected 1g on Z-axis
        
        gyro_offsets[0] += gyro[0];
        gyro_offsets[1] += gyro[1];
        gyro_offsets[2] += gyro[2];
        
        delay_ms(10);
    }
    
    // Calculate averages
    for i in 0..3 {
        accel_offsets[i] /= SAMPLES as f32;
        gyro_offsets[i] /= SAMPLES as f32;
    }
    
    println!("Calibration complete!");
    println!("Accel offsets: [{:.4}, {:.4}, {:.4}]", accel_offsets[0], accel_offsets[1], accel_offsets[2]);
    println!("Gyro offsets: [{:.4}, {:.4}, {:.4}]", gyro_offsets[0], gyro_offsets[1], gyro_offsets[2]);
    
    Ok([accel_offsets[0], accel_offsets[1], accel_offsets[2], 
        gyro_offsets[0], gyro_offsets[1], gyro_offsets[2]])
}
```

### Orientation Estimation

```rust
use hayasen::prelude::*;

fn estimate_orientation() -> Result<(), Error<YourI2cError>> {
    let i2c = setup_i2c();
    let mut sensor = Mpu9250::new(i2c, 0x68);
    sensor.initialize_sensor(AccelRange::Range2G, GyroRange::Range250Dps)?;
    
    loop {
        let accel = sensor.read_acceleration()?;
        
        // Calculate roll and pitch from accelerometer (when stationary)
        let roll = accel[1].atan2(accel[2]) * 180.0 / core::f32::consts::PI;
        let pitch = (-accel[0]).atan2((accel[1].powi(2) + accel[2].powi(2)).sqrt()) 
                   * 180.0 / core::f32::consts::PI;
        
        println!("Roll: {:.2}°, Pitch: {:.2}°", roll, pitch);
        
        delay_ms(100);
    }
}
```

### Multi-Sensor Setup

```rust
use hayasen::prelude::*;

fn multiple_sensors_example() -> Result<(), Error<YourI2cError>> {
    let i2c = setup_i2c();
    
    // MPU9250 can have two possible I2C addresses
    let mut sensor1 = Mpu9250::new(i2c, 0x68); // AD0 = LOW
    let mut sensor2 = Mpu9250::new(i2c, 0x69); // AD0 = HIGH
    
    // Initialize both sensors
    sensor1.initialize_sensor(AccelRange::Range2G, GyroRange::Range250Dps)?;
    sensor2.initialize_sensor(AccelRange::Range2G, GyroRange::Range250Dps)?;
    
    loop {
        let accel1 = sensor1.read_acceleration()?;
        let accel2 = sensor2.read_acceleration()?;
        
        println!("Sensor 1: [{:.3}, {:.3}, {:.3}]g", accel1[0], accel1[1], accel1[2]);
        println!("Sensor 2: [{:.3}, {:.3}, {:.3}]g", accel2[0], accel2[1], accel2[2]);
        
        delay_ms(100);
    }
}
```

## Best Practices

### Initialization Checklist

1. **Always verify sensor identity** before configuration
2. **Configure power management** early in initialization
3. **Set appropriate ranges** based on your application requirements
4. **Configure sample rate** to match your update frequency needs
5. **Handle errors gracefully** with appropriate retry logic

### Sample Rate Guidelines

```rust
// Sample rate formula: Sample Rate = 1000Hz / (1 + SMPRT_DIV)
sensor.set_sample_rate(0)?;   // 1000 Hz - High frequency applications
sensor.set_sample_rate(9)?;   // 100 Hz - General motion tracking
sensor.set_sample_rate(19)?;  // 50 Hz - Low power applications
sensor.set_sample_rate(99)?;  // 10 Hz - Very low power monitoring
```

### Memory Usage Optimization

```rust
use hayasen::prelude::*;

// For memory-constrained systems, use raw readings when possible
fn memory_efficient_reading() -> Result<(), Error<YourI2cError>> {
    let i2c = setup_i2c();
    let mut sensor = Mpu9250::new(i2c, 0x68);
    sensor.initialize_sensor(AccelRange::Range2G, GyroRange::Range250Dps)?;
    
    // Read raw data to avoid floating-point operations
    let raw_accel = sensor.read_accel_raw()?;
    
    // Manual scaling only when needed
    const SCALE_2G: f32 = 2.0 / 32768.0;
    let scaled_x = raw_accel[0] as f32 * SCALE_2G;
    
    Ok(())
}
```

## Troubleshooting

### Common Issues and Solutions

**Sensor Not Detected:**
- Check I2C wiring (SDA, SCL, VCC, GND)
- Verify I2C address (0x68 or 0x69 depending on AD0 pin)
- Ensure proper pull-up resistors on I2C lines

**Inconsistent Readings:**
- Check power supply stability
- Verify sample rate configuration
- Consider using digital low-pass filter
- Ensure sensor is properly mounted

**High Noise:**
- Lower the digital low-pass filter bandwidth
- Reduce sample rate if appropriate
- Check for electromagnetic interference
- Implement software filtering

### Debug Helper Functions

```rust
use hayasen::prelude::*;

fn debug_sensor_status(sensor: &mut Mpu9250<impl I2c>) -> Result<(), Error<impl std::fmt::Debug>> {
    // Verify sensor is responding
    match sensor.verify_identity() {
        Ok(_) => println!("✓ Sensor identity verified"),
        Err(e) => println!("✗ Identity check failed: {:?}", e),
    }
    
    // Test basic readings
    match sensor.read_temp_raw() {
        Ok(temp) => println!("✓ Raw temperature reading: {}", temp),
        Err(e) => println!("✗ Temperature read failed: {:?}", e),
    }
    
    Ok(())
}
```

This documentation provides comprehensive examples for using the MPU9250 library across different scenarios
and platforms. The examples progress from simple usage to advanced applications, helping developers implement
motion sensing in their embedded projects effectively.
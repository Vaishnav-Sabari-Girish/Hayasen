# Development Notes - Hayasen Crate

## Crate Structure

### Module Organization

The crate follows a modular design pattern optimized for embedded systems:

```
hayasen/
├── src/
│   ├── lib.rs          # Main library entry point
│   ├── error.rs        # Unified error types
│   ├── functions.rs    # Function registry system
│   └── mpu9250.rs      # MPU9250 sensor implementation
├── examples/           # Usage examples
└── tests/              # Integration tests
```

### Key Architectural Decisions

#### 1. Feature-Gated Compilation

```rust
// lib.rs
#[cfg(feature = "mpu9250")]
pub mod mpu9250;

#[cfg(feature = "mpu9250")]
pub use crate::mpu9250::*;
```

**Rationale:** The crate uses conditional compilation to include only the sensors you need, reducing binary size and compilation time. This is crucial for resource-constrained embedded systems.

**Usage:**
```toml
[dependencies]
hayasen = { version = "x.x.x", features = ["mpu9250"] }
```

#### 2. Prelude Pattern

```rust
pub mod prelude {
    pub use crate::error::Error;
    #[cfg(feature = "mpu9250")]
    pub use crate::mpu9250::*;
    pub use embedded_hal::i2c::I2c;
}
```

**Purpose:** Provides a convenient way to import commonly used types and traits. Users can simply `use hayasen::prelude::*;` to get everything they need.

#### 3. Convenience Layer (`mpu9250_hayasen`)

```rust
pub mod mpu9250_hayasen {
    // Simplified API for common operations
    pub fn create_default<I2C, E>(i2c: I2C, address: u8) -> Result<mpu9250::Mpu9250<I2C>, Error<E>>
    pub fn read_all<I2C, E>(sensor: &mut mpu9250::Mpu9250<I2C>) -> Result<(f32, [f32; 3], [f32; 3]), Error<E>>
}
```

**Benefits:**
- Reduces boilerplate for common operations
- Provides sensible defaults for new users
- Maintains access to low-level API for advanced users

### Driver Architecture

#### Core Driver Structure

```rust
pub struct Mpu9250<I2C> {
    i2c: I2C,           // Owned I2C peripheral
    address: u8,        // Device I2C address
    accel_scale: f32,   // Cached scaling factor for acceleration
    gyro_scale: f32,    // Cached scaling factor for angular velocity
}
```

**Design Principles:**
- **Ownership Model:** The driver takes ownership of the I2C peripheral to prevent conflicts
- **Zero-Cost Abstractions:** Scale factors are pre-calculated and cached to avoid runtime divisions
- **Type Safety:** Generic over I2C type for platform independence

#### State Management

The driver maintains minimal state to reduce memory footprint:
- Scale factors are calculated once during configuration
- No internal buffering or filtering (left to user applications)
- Stateless register operations for maximum flexibility

## `no_std` Compatibility

### Core Requirements

```rust
#![no_std]
#![no_main]  // For applications, not the library itself
```

### Memory Management

#### No Dynamic Allocation
```rust
// ✓ Stack-allocated arrays
let mut buffer = [0u8; 6];

// ✗ Avoid heap allocation
// let mut buffer = vec![0u8; 6];  // This would require std
```

#### Fixed-Size Buffers
All I2C operations use compile-time sized buffers:
```rust
pub fn read_accel_raw(&mut self) -> Result<[i16; 3], Error<E>> {
    let mut buffer = [0u8; 6];  // Fixed size, stack allocated
    // ...
}
```

### Core Library Dependencies

The crate only depends on `core` and `embedded-hal`:
```rust
use core::fmt::{Debug, Formatter, Result};  // ✓ Core library
use embedded_hal::i2c::I2c;                  // ✓ Hardware abstraction

// ✗ Avoid std dependencies
// use std::vec::Vec;
// use std::collections::HashMap;
```

### Floating Point Considerations

#### Target Compatibility
```rust
// The crate uses f32 for sensor data conversion
let temperature = (raw as f32) / 340.0 + 36.53;
```

**Notes:**
- Uses `f32` for better performance on 32-bit ARM Cortex-M
- All calculations are optimized for embedded floating-point units
- For targets without FPU, consider using fixed-point arithmetic wrapper

#### Alternative for No-FPU Targets
```rust
// Example fixed-point implementation (not included in crate)
pub fn read_acceleration_fixed(&mut self) -> Result<[i32; 3], Error<E>> {
    let raw = self.read_accel_raw()?;
    // Scale to milligee (1/1000 g) using integer math
    let scale_factor = (self.accel_scale * 1000.0) as i32;
    Ok([
        (raw[0] as i32 * scale_factor) / 1000,
        (raw[1] as i32 * scale_factor) / 1000,
        (raw[2] as i32 * scale_factor) / 1000,
    ])
}
```

### Const Generics and Compile-Time Optimization

```rust
// Register addresses are compile-time constants
const WHO_AM_I: u8 = 0x75;
const ACCEL_XOUT_H: u8 = 0x3B;

// Enums are zero-cost abstractions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccelRange {
    Range2G,    // Compiles to simple integer values
    Range4G,
    Range8G,
    Range16G,
}
```

## Safety Considerations

### I2C Communication Safety

#### Transaction Atomicity
```rust
pub fn read_accel_raw(&mut self) -> Result<[i16; 3], Error<E>> {
    let mut buffer = [0u8; 6];
    self.i2c.write(self.address, &[ACCEL_XOUT_H])?;  // Register pointer
    self.i2c.read(self.address, &mut buffer)?;       // Read data
    // ... process buffer
}
```

**Critical Points:**
- **Atomic Operations:** Each register read is a complete write-then-read transaction
- **Error Propagation:** I2C errors are immediately propagated using `?` operator
- **Buffer Safety:** Fixed-size buffers prevent overflow issues

#### Address Validation

```rust
pub fn verify_identity(&mut self) -> Result<(), Error<E>> {
    let mut buffer = [0u8];
    self.i2c.write(self.address, &[WHO_AM_I])?;
    self.i2c.read(self.address, &mut buffer)?;
    if buffer[0] != WHO_AM_I_VALUE {
        return Err(Error::NotDetected);  // Explicit validation
    }
    Ok(())
}
```

**Safety Features:**
- **Device Verification:** Always check WHO_AM_I register before operation
- **Type Safety:** Rust's type system prevents incorrect address usage
- **Explicit Error States:** Clear error types for different failure modes

### Memory Safety

#### Buffer Bounds Checking

```rust
// Safe array access - Rust prevents buffer overruns at compile time
let x = ((buffer[0] as i16) << 8) | buffer[1] as i16;  // Safe indexing
let y = ((buffer[2] as i16) << 8) | buffer[3] as i16;  // Bounds checked
let z = ((buffer[4] as i16) << 8) | buffer[5] as i16;  // Compile-time verified
```

#### Integer Overflow Protection

```rust
// Safe conversion with explicit casting
let x = raw[0] as f32 * self.accel_scale;  // Explicit type conversion

// For temperature calculation, use checked arithmetic in critical applications
pub fn read_temperature_celsius_safe(&mut self) -> Result<f32, Error<E>> {
    let raw = self.read_temp_raw()?;
    let temp_f64 = (raw as f64) / 340.0 + 36.53;  // Higher precision intermediate
    Ok(temp_f64 as f32)
}
```

### Register Access Safety

#### Read-Modify-Write Operations
```rust
pub fn wake_up(&mut self) -> Result<(), Error<E>> {
    let mut buffer = [0u8];
    self.i2c.write(self.address, &[PWR_MGMT_1])?;     // Read current value
    self.i2c.read(self.address, &mut buffer)?;
    let new_config = buffer[0] & 0xBF;                // Clear sleep bit safely
    self.i2c.write(self.address, &[PWR_MGMT_1, new_config])?;  // Write back
    Ok(())
}
```

**Safety Measures:**
- **Atomic RMW:** Complete read-modify-write sequence
- **Bit Masking:** Safe bit manipulation using explicit masks
- **State Preservation:** Only modify intended bits, preserve others

### Error Handling Strategy

#### Comprehensive Error Types

```rust
#[derive(Clone, PartialEq, Eq)]
pub enum Error<E> {
    I2c(E),                    // Underlying I2C errors
    InvalidData,               // Data validation failures
    NotDetected,              // Device not found/responding
    ConfigError,              // Configuration parameter errors
    SensorSpecific(&'static str), // Sensor-specific error messages
}
```

#### Error Recovery Patterns

```rust
// Recommended error handling pattern
fn safe_sensor_operation() -> Result<(), Error<YourI2cError>> {
    let mut retry_count = 0;
    const MAX_RETRIES: u8 = 3;
    
    loop {
        match sensor.read_acceleration() {
            Ok(data) => return Ok(process_data(data)),
            Err(Error::I2c(_)) if retry_count < MAX_RETRIES => {
                retry_count += 1;
                delay_ms(10);  // Brief delay before retry
                continue;
            },
            Err(e) => return Err(e),  // Propagate non-recoverable errors
        }
    }
}
```

### Concurrency Safety

#### Single-Threaded Design
```rust
// The driver requires &mut self for all operations
impl<I2C, E> Mpu9250<I2C> where I2C: I2c<Error = E> {
    pub fn read_acceleration(&mut self) -> Result<[f32; 3], Error<E>>
    //                        ^^^^ Exclusive access required
}
```

**Implications:**
- **Thread Safety:** Not `Send` or `Sync` by default - I2C peripherals are typically not thread-safe
- **Exclusive Access:** Prevents concurrent access to I2C bus
- **RAII Pattern:** Resource cleanup handled by Rust's ownership system

#### Interrupt Safety

```rust
// For interrupt-driven applications
fn interrupt_safe_reading() {
    // Disable interrupts during I2C transaction if required by your platform
    critical_section::with(|_cs| {
        let result = sensor.read_acceleration();
        // Process result...
    });
}
```

### Performance Considerations

#### Zero-Copy Design

```rust
// Direct array return - no heap allocation
pub fn read_accel_raw(&mut self) -> Result<[i16; 3], Error<E>> {
    // Returns stack-allocated array directly
}
```

#### Minimal Register Access

```rust
// Efficient register reading - single transaction for multiple axes
let mut buffer = [0u8; 6];  // Read all 6 bytes (3 axes × 2 bytes) at once
self.i2c.write(self.address, &[ACCEL_XOUT_H])?;
self.i2c.read(self.address, &mut buffer)?;
```

### Development Workflow

#### Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock I2C for unit testing
    struct MockI2c {
        expected_writes: Vec<Vec<u8>>,
        read_responses: Vec<Vec<u8>>,
    }
    
    impl I2c for MockI2c {
        type Error = ();
        
        fn write(&mut self, _addr: u8, data: &[u8]) -> Result<(), Self::Error> {
            // Verify expected write operations
            Ok(())
        }
        
        fn read(&mut self, _addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
            // Return mock data
            Ok(())
        }
    }
    
    #[test]
    fn test_sensor_initialization() {
        let mock_i2c = MockI2c::new();
        let mut sensor = Mpu9250::new(mock_i2c, 0x68);
        // Test initialization logic...
    }
}
```

#### Documentation Guidelines

```rust
/// Reads accelerometer data in g-force units
/// 
/// # Returns
/// 
/// Array of [X, Y, Z] acceleration values in g-force units.
/// The scaling is automatically applied based on the configured range.
/// 
/// # Errors
/// 
/// Returns `Error::I2c` if communication fails.
/// 
/// # Example
/// 
/// ```no_run
/// use hayasen::prelude::*;
/// 
/// let mut sensor = Mpu9250::new(i2c, 0x68);
/// sensor.initialize_sensor(AccelRange::Range2G, GyroRange::Range250Dps)?;
/// let accel = sensor.read_acceleration()?;
/// println!("Acceleration: {:?}", accel);
/// ```
pub fn read_acceleration(&mut self) -> Result<[f32; 3], Error<E>>
```

### Platform Integration Notes

#### Embedded HAL Compatibility

```rust
// Generic over any I2C implementation
impl<I2C, E> Mpu9250<I2C>
where 
    I2C: I2c<Error = E>  // Uses embedded-hal trait
{
    // Platform-agnostic implementation
}
```

**Benefits:**
- **Platform Independence:** Works with any embedded-hal compliant I2C driver
- **Type Safety:** Compile-time verification of I2C compatibility
- **Error Propagation:** Preserves underlying platform error types

#### Memory Layout Considerations

```rust
#[repr(C)]
struct SensorData {
    temperature: f32,    // 4 bytes
    accel: [f32; 3],    // 12 bytes
    gyro: [f32; 3],     // 12 bytes
}  // Total: 28 bytes - predictable layout
```

### Safety Patterns

#### Resource Management

```rust
// RAII pattern ensures I2C is properly released
impl<I2C> Drop for Mpu9250<I2C> {
    fn drop(&mut self) {
        // I2C peripheral is automatically returned when Mpu9250 is dropped
        // No explicit cleanup required due to Rust's ownership system
    }
}
```

#### Initialization Safety

```rust
pub fn initialize_sensor(&mut self, accel_range: AccelRange, gyro_range: GyroRange) -> Result<(), Error<E>> {
    self.verify_identity()?;        // Always verify device first
    self.configure_power()?;        // Ensure proper power state
    self.setup_accelerometer(accel_range)?;  // Configure before use
    self.setup_gyroscope(gyro_range)?;       // Configure before use
    Ok(())
}
```

**Safety Chain:**
1. Device identity verification prevents wrong device communication
2. Power configuration ensures device is ready
3. Sensor configuration sets known state before operation

#### Register Access Patterns

```rust
// Safe register write pattern
fn write_register(&mut self, register: u8, value: u8) -> Result<(), Error<E>> {
    // Always write register address followed by data
    self.i2c.write(self.address, &[register, value])?;
    Ok(())
}

// Safe register read pattern  
fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
    let mut buffer = [0u8; 1];
    self.i2c.write(self.address, &[register])?;      // Set register pointer
    self.i2c.read(self.address, &mut buffer)?;       // Read data
    Ok(buffer[0])
}
```

### Error Safety Guarantees

#### Fail-Fast Design

```rust
// Sensor operations fail immediately on error
pub fn read_acceleration(&mut self) -> Result<[f32; 3], Error<E>> {
    let raw = self.read_accel_raw()?;  // Fail fast on I2C error
    // Only proceed if raw read succeeded
    let x = raw[0] as f32 * self.accel_scale;
    // ...
}
```

#### Error Propagation Chain

```rust
// Error chain: I2C Error -> Driver Error -> Application Error
Hardware I2C Error
    ↓ (From<E> implementation)
Driver Error<E>
    ↓ (? operator)  
Application Error Handling
```

### Integration Patterns

#### Dependency Injection

```rust
// The driver doesn't create its own I2C - it's injected
let i2c = platform_specific_i2c_setup();
let sensor = Mpu9250::new(i2c, address);  // Dependency injection
```

**Benefits:**
- Testability with mock I2C implementations
- Platform flexibility
- Resource sharing control

#### Builder Pattern Extension

```rust
// Example extension for complex configurations
impl<I2C, E> Mpu9250<I2C> 
where I2C: I2c<Error = E>
{
    pub fn builder(i2c: I2C, address: u8) -> SensorBuilder<I2C> {
        SensorBuilder::new(i2c, address)
    }
}

pub struct SensorBuilder<I2C> {
    sensor: Mpu9250<I2C>,
    configured: bool,
}

impl<I2C, E> SensorBuilder<I2C> 
where I2C: I2c<Error = E>
{
    pub fn with_accel_range(mut self, range: AccelRange) -> Result<Self, Error<E>> {
        self.sensor.setup_accelerometer(range)?;
        Ok(self)
    }
    
    pub fn build(self) -> Result<Mpu9250<I2C>, Error<E>> {
        if !self.configured {
            return Err(Error::ConfigError);
        }
        Ok(self.sensor)
    }
}
```

### Debugging and Development

#### Register Debug Utilities

```rust
#[cfg(debug_assertions)]
impl<I2C, E> Mpu9250<I2C>
where I2C: I2c<Error = E>
{
    pub fn dump_registers(&mut self) -> Result<(), Error<E>> {
        let registers = [WHO_AM_I, PWR_MGMT_1, ACCEL_CONFIG, GYRO_CONFIG];
        
        for &reg in &registers {
            match self.read_register(reg) {
                Ok(value) => println!("Register 0x{:02X}: 0x{:02X}", reg, value),
                Err(e) => println!("Failed to read register 0x{:02X}: {:?}", reg, e),
            }
        }
        Ok(())
    }
}
```

#### Compile-Time Configuration Validation

```rust
// Use const assertions for compile-time validation
const _: () = {
    assert!(WHO_AM_I_VALUE == 0x71, "Incorrect WHO_AM_I value for MPU9250");
};
```

### Performance Optimization Notes

#### Batch Operations

```rust
// Reading all axes in single I2C transaction is more efficient
pub fn read_accel_raw(&mut self) -> Result<[i16; 3], Error<E>> {
    let mut buffer = [0u8; 6];  // Read all 3 axes at once
    // More efficient than 3 separate 2-byte reads
}
```

#### Cache-Friendly Access Patterns

```rust
// Scale factors are cached to avoid repeated calculations
impl<I2C, E> Mpu9250<I2C> {
    pub fn setup_accelerometer(&mut self, range: AccelRange) -> Result<(), Error<E>> {
        let (config_value, scale) = match range {
            AccelRange::Range2G => (0x00, 2.0 / 32768.0),  // Pre-calculated
            // ...
        };
        self.accel_scale = scale;  // Cache for future use
        Ok(())
    }
}
```

### Future Development Considerations

#### Extensibility

- **Modular Design:** Easy to add new sensor support via feature flags
- **Trait Abstractions:** Common sensor operations could be abstracted into traits
- **Async Support:** Could be extended for async I2C operations

#### Version Compatibility

```rust
// Use semantic versioning for breaking changes
// Major: Breaking API changes
// Minor: New features, backwards compatible  
// Patch: Bug fixes only
```

The crate is designed with embedded systems' constraints in mind, prioritizing safety, performance, and resource efficiency while maintaining a clean, intuitive API.
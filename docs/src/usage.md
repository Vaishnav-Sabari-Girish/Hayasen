# Usage

## 📦 Installation

Add `hayasen` to your `Cargo.toml` either by the below command or directly.

```bash
# Command Line
cargo add hayasen --features mpu9250    #Optional for now
```
`Cargo.toml`

```toml
[dependencies]
hayasen = { version = "*", features = ["mpu9250"] }   # For MPU9250 support
```


## 🎯 Quick Start


Here is a basic example of `hayasen` (Not Yet Tested on actual Hardware)

```rust
use hayasen::prelude::*;
use embedded_hal::i2c::I2c;

fn main() -> Result<(), Error<MyI2CError>> {
    // Easy initialization with default settings
    let mut mpu = HayasenFunctions::create_mpu9250_default(i2c, 0x68)?;
    
    // Read sensor data
    let acceleration = HayasenFunctions::read_accel(&mut mpu)?;
    let temperature = HayasenFunctions::read_temp_c(&mut mpu)?;
    
    println!("Temperature: {:.1}°C", temperature);
    println!("Acceleration: {:?} g", acceleration);
    
    Ok(())
}
```
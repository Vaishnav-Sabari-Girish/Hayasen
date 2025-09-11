---
title: Hayasen
sub_title : A Robust Embedded Rust Library for Multi-Sensor Integration
theme : 
    name : tokyonight-storm
    override : 
        footer : 
            style: template
            left : "@Vaishnav-Sabari-Girish"
            right : "ヴァイシュナヴ。サバリ。ギリシュ"
author : Vaishnav Sabari Girish
---


# WHO_AM_I

<!--column_layout: [1, 3]-->
<!--column: 0--> 

<!--new_lines: 5-->
![image:width:100%](./me.png)

<!--column: 1-->

```rust
fn main() {
  let WHO_AM_I: &str = "Vaishnav Sabari Girish";

  let designation: &str = "Final Year ECE Student";

  let mut interests: Vec<&str> = vec![
      "Open Source",
      "Embedded Systems",
      "Rust",
      "CLI/TUI"
  ];

  let mut projects: Vec<&str> = vec![
      // Rust Projects
      "Hayasen",
      "ComChan (Serial Monitor)",
      "Arduino CLI Interactive",   // Except This one
  ];

  let website: &str = "https://vaishnav.world";
}
```
<!--end_slide-->

# Introduction to Hayasen

---

![image:width:30%](./logo_no_bg.png)

--- 

1. `hayasen` is an Embedded Rust library that supports multiple sensors for Embedded Systems.
2. It provides a type-safe and unified API to interact with various sensors.
3. Currently supports **MPU9250** and **MPU6050** IMU's. (Only I2C)
4. Support for more sensors on the way.

## Links

1. Github (Main repo) : [](https://github.com/Vaishnav-Sabari-Girish/Hayasen)
2. Github (Examples repo) : [](https://github.com/Vaishnav-Sabari-Girish/Hayasen-Examples)


<!--new_lines: 1-->
<!--alignment: center-->
**Contributions are Welcome**

<!--end_slide-->

# `Cargo.toml`

```toml
[package]
name = "hayasen"
version = "0.0.8"
edition = "2024"
description = "A multi sensor library for Embedded Rust supporting multiple sensors"
authors = ["Vaishnav-Sabari-Girish forgamesonly684@gmail.com"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/Vaishnav-Sabari-Girish/Hayasen"
documentation = "http://vaishnav.world/Hayasen/"
keywords = ["embedded", "sensors", "driver", "no-std"]
categories = ["embedded", "no-std", "hardware-support"]
readme = "README.md"


[dependencies]
embedded-hal = "1.0.0"

[features]
default = []
mpu9250 = []
mpu6050 = []
```

<!--end_slide-->

# Project `src/` directory

```bash +exec
tree src/
```

<!--end_slide-->

# Project Structure 

```mermaid +render +width:100%
graph TD 
    driver_file(["`sensor_name.rs`" file])
    functions_file(["`functions.rs`" file])
    lib_file(["`lib.rs`" file])
    error_file(["`error.rs`" file])
    driver_file --Function Pointers--> functions_file --> lib_file
    driver_file <--> error_file
```

<!--end_slide-->

# Usage - 1

## Adding the library using `cargo add`

```bash
cargo add hayasen --features mpu6050 # OR mpu9250
```

<!--new_lines: 1-->

## Adding it to `Cargo.toml`

```toml
[package]
name = "<project_name>"
version = "0.1.0"
edition = "2024"

[dependencies]
embedded-hal = "1.0.0"
hayasen = { version = "0.0.8" , features = ["mpu6050"] }   # OR mpu9250
```

<!--end_slide-->

# Usage - 2 

## Example program (Generic) for MPU9250

```rust
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

<!--end_slide-->

# Outputs - 1

## MPU9250

![image:width:100%](./examples/MPU9250/basic_mpu9250/basic_mpu9250.gif)

<!--end_slide-->

# Outputs - 2

## MPU6050

![image:width:100%](./examples/MPU6050/mpu6050_nrf/out.gif)

<!--end_slide-->

# Why Rust ?

The below points explain why I have used Rust to create `hayasen`

1. Robust **Error Handling** using `Result<>`
<!--new_lines: 1-->
2. Memory safety via **Ownership/Borrowing** avoid race conditions and undefined behaviour.

<!--new_lines: 1-->
3. Predictable performance from zero-cost abstractions suits real-time constraints.

<!--new_lines: 1-->
4. `no_std` support enables bare-metal firmware without OS or `std`.

<!--new_lines: 1-->
5. Portable drivers through `embedded-hal` traits across MCU's/HAL's

<!--new_lines: 1-->
6. **Ecosystem Gaps** : Some sensor crates are outdated or missing. Which is where `hayasen` comes in.

<!--new_lines: 1-->
7. Strong tooling (`cargo`, docs) speeds development and testing.

<!--end_slide-->

![image:width:100%](./ty.png)

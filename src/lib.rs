#![no_std]
#![no_main]


//! Example usage documentation
//! # Examples
//!
//! ```rust,ignore
//! use hayasen::prelude::*;
//! use embedded_hal::i2c::I2c;
//!
//! fn main() -> Result<(), Error<MyI2CError>> {
//!     // Easy initialization
//!     let mut mpu = HayasenFunctions::create_mpu9250_default(i2c, 0x68)?;
//!
//!     // Quick readings 
//!     let acceleration = HayasenFunctions::read_acceleration(&mut mpu)?;
//!     let gyroscope = HayasenFunctions::read_angular_velocity(&mut mpu)?;
//!
//!     // Or read everything at once
//!     let (temp, accel, gyro) = HayasenFunctions::read_all(&mut mpu)?;
//!
//!     Ok(())
//! }
//!
//! ```


pub mod error;
pub mod functions;
pub mod mpu9250;

pub use error::Error;
pub use functions::HayasenFunctions;

// Prelude module for convinient imports
pub mod prelude {
    pub use crate::error::Error;
    pub use crate::functions::HayasenFunctions;
    pub use crate::mpu9250::{Mpu9250, AccelRange, GyroRange, DlpfConfig};
    pub use embedded_hal::i2c::I2c;
}

// Wrapper function for MPU9250 
#[cfg(feature = "mpu9250")]
impl<I2C, E> HayasenFunctions<I2C, E>  
where 
    I2C : embedded_hal::i2c::I2c<Error = E>
{
    pub fn create_mpu9250_default(i2c : I2C, address : u8) -> Result<mpu9250::Mpu9250<I2C>, Error<E>> {
        let mut sensor = mpu9250::Mpu9250::new(i2c, address);
        sensor.initialize_sensor(
            mpu9250::AccelRange::Range2G,
            mpu9250::GyroRange::Range250Dps,
        )?;
        Ok(sensor)
    }

    pub fn read_accel(sensor : &mut mpu9250::Mpu9250<I2C>) -> Result<[f32; 3], Error<E>> {
        let accel_fun = HayasenFunctions::new();
        (accel_fun.mpu9250.read_acceleration)(sensor)
    }

    pub fn read_gyro(sensor : &mut mpu9250::Mpu9250<I2C>) -> Result<[f32; 3], Error<E>> {
        let gyro_fun = HayasenFunctions::new();
        (gyro_fun.mpu9250.read_angular_velocity)(sensor)
    }

    pub fn read_temp_c(sensor : &mut mpu9250::Mpu9250<I2C>) -> Result<f32, Error<E>> {
        let temp_fun = HayasenFunctions::new();
        (temp_fun.mpu9250.read_temperature_celsius)(sensor)
    }

    // Read all data at once (optional)
    pub fn read_all(sensor : &mut mpu9250::Mpu9250<I2C>) -> Result<(f32, [f32; 3], [f32; 3]), Error<E>> {
        let all_fun = HayasenFunctions::new();
        let temp = (all_fun.mpu9250.read_temperature_celsius)(sensor)?;
        let accel = (all_fun.mpu9250.read_acceleration)(sensor)?;
        let gyro = (all_fun.mpu9250.read_angular_velocity)(sensor)?;
        Ok((temp, accel, gyro))
    }
}



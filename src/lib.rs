#![no_std]
#![no_main]

pub mod error;
pub mod mpu9250;

pub use error::Error;

pub mod prelude {
    pub use crate::error::Error;
    #[cfg(feature = "mpu9250")]
    pub use crate::mpu9250::*;
    pub use embedded_hal::i2c::I2c;
}

#[cfg(feature = "mpu9250")]
pub mod mpu9250_hayasen {
    use super::mpu9250;
    use super::error::Error;
    use embedded_hal::i2c::I2c;

    pub fn create_default<I2C, E>(i2c: I2C, address: u8) -> Result<mpu9250::Mpu9250<I2C>, Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        let mut sensor = mpu9250::Mpu9250::new(i2c, address);
        sensor.initialize_sensor(
            mpu9250::AccelRange::Range2G,
            mpu9250::GyroRange::Range250Dps,
        )?;
        Ok(sensor)
    }

    pub fn read_acceleration<I2C, E>(sensor: &mut mpu9250::Mpu9250<I2C>) -> Result<[f32; 3], Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.read_acceleration()
    }

    pub fn read_angular_velocity<I2C, E>(sensor: &mut mpu9250::Mpu9250<I2C>) -> Result<[f32; 3], Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.read_angular_velocity()
    }

    pub fn read_temperature<I2C, E>(sensor: &mut mpu9250::Mpu9250<I2C>) -> Result<f32, Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.read_temperature_celsius()
    }

    pub fn read_all<I2C, E>(sensor: &mut mpu9250::Mpu9250<I2C>) -> Result<(f32, [f32; 3], [f32; 3]), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        let temp = sensor.read_temperature_celsius()?;
        let accel = sensor.read_acceleration()?;
        let gyro = sensor.read_angular_velocity()?;
        Ok((temp, accel, gyro))
    }
}
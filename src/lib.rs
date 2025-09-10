#![no_std]
#![no_main]

pub mod error;
pub mod mpu9250;
pub mod mpu6050;
pub use error::Error;

pub mod prelude {
    pub use crate::error::Error;
    #[cfg(feature = "mpu9250")]
    pub use crate::mpu9250;

    #[cfg(feature = "mpu6050")]
    pub use crate::mpu6050;
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

#[cfg(feature = "mpu6050")]
pub mod mpu6050_hayasen {
    use super::mpu6050;
    use super::error::Error;
    use embedded_hal::i2c::I2c;

    pub fn create_default<I2C, E>(i2c: I2C, address: u8) -> Result<mpu6050::Mpu6050<I2C>, Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        let mut sensor = mpu6050::Mpu6050::new(i2c, address);
        sensor.initialize_sensor(
            mpu6050::AccelRange::Range2G,
            mpu6050::GyroRange::Range250Dps,
        )?;
        Ok(sensor)
    }

    pub fn create_default_with_config<I2C, E>(
        i2c: I2C, 
        address: u8, 
        accel_range: mpu6050::AccelRange, 
        gyro_range: mpu6050::GyroRange
    ) -> Result<mpu6050::Mpu6050<I2C>, Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        let mut sensor = mpu6050::Mpu6050::new(i2c, address);
        sensor.initialize_sensor(accel_range, gyro_range)?;
        Ok(sensor)
    }

    pub fn read_acceleration<I2C, E>(sensor: &mut mpu6050::Mpu6050<I2C>) -> Result<[f32; 3], Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.read_acceleration()
    }

    pub fn read_angular_velocity<I2C, E>(sensor: &mut mpu6050::Mpu6050<I2C>) -> Result<[f32; 3], Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.read_angular_velocity()
    }

    pub fn read_temperature<I2C, E>(sensor: &mut mpu6050::Mpu6050<I2C>) -> Result<f32, Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.read_temperature_celsius()
    }

    pub fn read_all<I2C, E>(sensor: &mut mpu6050::Mpu6050<I2C>) -> Result<(f32, [f32; 3], [f32; 3]), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        let temp = sensor.read_temperature_celsius()?;
        let accel = sensor.read_acceleration()?;
        let gyro = sensor.read_angular_velocity()?;
        Ok((temp, accel, gyro))
    }

    // Additional MPU6050-specific convenience functions
    pub fn setup_low_power_mode<I2C, E>(sensor: &mut mpu6050::Mpu6050<I2C>) -> Result<(), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.set_dlpf_config(mpu6050::DlpfConfig::Bandwidth5Hz)?;
        sensor.set_sample_rate(199)?; // 5Hz sample rate (1000Hz/(199+1))
        Ok(())
    }

    pub fn setup_high_performance_mode<I2C, E>(sensor: &mut mpu6050::Mpu6050<I2C>) -> Result<(), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.set_dlpf_config(mpu6050::DlpfConfig::Bandwidth260Hz)?;
        sensor.set_sample_rate(7)?; // 125Hz sample rate (1000Hz/(7+1))
        Ok(())
    }

    pub fn disable_temperature_save_power<I2C, E>(sensor: &mut mpu6050::Mpu6050<I2C>) -> Result<(), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.disable_temperature_sensor()
    }

    pub fn enable_temperature<I2C, E>(sensor: &mut mpu6050::Mpu6050<I2C>) -> Result<(), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.enable_temperature_sensor()
    }
}

#![no_std]
#![no_main]

pub mod error;

#[cfg(feature = "mpu9250")]
pub mod mpu9250;

#[cfg(feature = "mpu6050")]
pub mod mpu6050;

#[cfg(feature = "max30102")]
pub mod max30102;

pub use error::Error;

pub mod prelude {
    pub use crate::error::Error;
    #[cfg(feature = "mpu9250")]
    pub use crate::mpu9250;

    #[cfg(feature = "mpu6050")]
    pub use crate::mpu6050;

    #[cfg(feature = "max30102")]
    pub use crate::max30102;
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

#[cfg(feature = "max30102")]
pub mod max30102_hayasen {
    use super::max30102;
    use super::error::Error;
    use embedded_hal::i2c::I2c;

    pub fn create_default<I2C, E>(i2c: I2C, address: u8) -> Result<max30102::Max30102<I2C>, Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        let mut sensor = max30102::Max30102::new(i2c, address);
        sensor.initialize_sensor()?;
        Ok(sensor)
    }

    pub fn create_default_with_address<I2C, E>(i2c: I2C) -> Result<max30102::Max30102<I2C>, Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        let mut sensor = max30102::Max30102::new_default(i2c);  // Fixed function name
        sensor.initialize_sensor()?;
        Ok(sensor)
    }

    pub fn create_heart_rate_mode<I2C, E>(i2c: I2C, address: u8) -> Result<max30102::Max30102<I2C>, Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        let mut sensor = max30102::Max30102::new(i2c, address);
        sensor.initialize_heart_rate_mode()?;
        Ok(sensor)
    }

    pub fn read_fifo_sample<I2C, E>(sensor: &mut max30102::Max30102<I2C>) -> Result<Option<max30102::FifoSample>, Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.read_fifo_sample()
    }

    pub fn read_fifo_batch<I2C, E>(sensor: &mut max30102::Max30102<I2C>, samples: &mut [max30102::FifoSample]) -> Result<usize, Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.read_fifo_batch(samples)
    }

    pub fn read_temperature<I2C, E>(sensor: &mut max30102::Max30102<I2C>) -> Result<Option<f32>, Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.read_temperature()
    }

    pub fn get_available_samples<I2C, E>(sensor: &mut max30102::Max30102<I2C>) -> Result<u8, Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.get_available_sample_count()
    }

    // Removed the read_all_available_samples function that used Vec

    // Additional MAX30102-specific convenience functions
    pub fn setup_low_power_mode<I2C, E>(sensor: &mut max30102::Max30102<I2C>) -> Result<(), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        // Configure for low power: lower sampling rate, higher averaging, lower LED power
        sensor.set_sampling_rate(max30102::SamplingRate::Rate50)?;
        sensor.set_sample_averaging(max30102::SampleAveraging::Average16)?;
        sensor.set_led_pulse_amplitude(1, 0x0F)?; // Lower Red LED power
        sensor.set_led_pulse_amplitude(2, 0x0F)?; // Lower IR LED power
        Ok(())
    }

    pub fn setup_high_performance_mode<I2C, E>(sensor: &mut max30102::Max30102<I2C>) -> Result<(), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        // Configure for high performance: higher sampling rate, lower averaging, higher LED power
        sensor.set_sampling_rate(max30102::SamplingRate::Rate400)?;
        sensor.set_sample_averaging(max30102::SampleAveraging::Average2)?;
        sensor.set_led_pulse_amplitude(1, 0x3F)?; // Higher Red LED power
        sensor.set_led_pulse_amplitude(2, 0x3F)?; // Higher IR LED power
        Ok(())
    }

    pub fn setup_proximity_detection<I2C, E>(sensor: &mut max30102::Max30102<I2C>, threshold: u8) -> Result<(), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.set_proximity_threshold(threshold)?;
        sensor.enable_interrupt(max30102::InterruptSource::AlcOverflow)?;
        Ok(())
    }

    pub fn enable_data_ready_interrupt<I2C, E>(sensor: &mut max30102::Max30102<I2C>) -> Result<(), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.enable_interrupt(max30102::InterruptSource::NewDataReady)
    }

    pub fn enable_fifo_interrupt<I2C, E>(sensor: &mut max30102::Max30102<I2C>, threshold: u8) -> Result<(), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.set_fifo_almost_full_threshold(threshold)?;
        sensor.enable_interrupt(max30102::InterruptSource::FifoAlmostFull)?;
        Ok(())
    }

    pub fn start_temperature_measurement<I2C, E>(sensor: &mut max30102::Max30102<I2C>) -> Result<(), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.start_temperature_measurement()
    }

    pub fn check_sensor_status<I2C, E>(sensor: &mut max30102::Max30102<I2C>) -> Result<(bool, max30102::OperationMode, (u8, u8)), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        let is_shutdown = sensor.is_shutdown()?;
        let mode = sensor.get_operation_mode()?;
        let interrupt_status = sensor.read_interrupt_status()?;
        Ok((is_shutdown, mode, interrupt_status))
    }

    pub fn reset_and_reinitialize<I2C, E>(sensor: &mut max30102::Max30102<I2C>) -> Result<(), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.force_reset()?;
        sensor.initialize_sensor()?;
        Ok(())
    }

    pub fn power_save_mode<I2C, E>(sensor: &mut max30102::Max30102<I2C>) -> Result<(), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.shutdown()
    }

    pub fn wake_from_power_save<I2C, E>(sensor: &mut max30102::Max30102<I2C>) -> Result<(), Error<E>>
    where
        I2C: I2c<Error = E>,
    {
        sensor.wakeup()
    }
}

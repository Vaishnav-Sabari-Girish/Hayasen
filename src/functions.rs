//Conditionally include features

#[cfg(feature = "mpu9250")]
pub mod mpu9250 {
    pub use crate::mpu9250::*;
}

use embedded_hal::i2c::I2c;
use crate::error::Error;

// Struct that holds function pointers for the MPU9250
#[cfg(feature = "mpu9250")]
pub struct MPU9250Functions<I2C, E> {
    pub verify_identity : fn(&mut mpu9250::Mpu9250<I2C>) -> Result<(), Error<E>>,
    pub configure_power : fn(&mut mpu9250::Mpu9250<I2C>) -> Result<(), Error<E>>,
    pub setup_accelerometer : fn(&mut mpu9250::Mpu9250<I2C>, mpu9250::AccelRange) -> Result<(), Error<E>>,
    pub setup_gyroscope : fn(&mut mpu9250::Mpu9250<I2C>, mpu9250::GyroRange) -> Result<(), Error<E>>,
    pub initialize_sensor : fn(&mut mpu9250::Mpu9250<I2C>, mpu9250::AccelRange, mpu9250::GyroRange) -> Result<(), Error<E>>,
    pub read_accel_raw : fn(&mut mpu9250::Mpu9250<I2C>) -> Result<[i16; 3], Error<E>>,
    pub read_gyro_raw : fn(&mut mpu9250::Mpu9250<I2C>) -> Result<[i16; 3], Error<E>>,
    pub read_temp_raw : fn(&mut mpu9250::Mpu9250<I2C>) -> Result<i16, Error<E>>,
    pub read_acceleration : fn(&mut mpu9250::Mpu9250<I2C>) -> Result<[f32; 3], Error<E>>,
    pub read_angular_velocity : fn(&mut mpu9250::Mpu9250<I2C>) -> Result<[f32; 3], Error<E>>,
    pub read_temperature_celsius : fn(&mut mpu9250::Mpu9250<I2C>) -> Result<f32, Error<E>>,
    pub set_sample_rate : fn(&mut mpu9250::Mpu9250<I2C>, u8) -> Result<(), Error<E>>,
    pub set_dlpf_config : fn(&mut mpu9250::Mpu9250<I2C>, mpu9250::DlpfConfig) -> Result<(), Error<E>>,
    pub enter_sleep_mode : fn(&mut mpu9250::Mpu9250<I2C>)-> Result<(), Error<E>>,
    pub wake_up : fn(&mut mpu9250::Mpu9250<I2C>) -> Result<(), Error<E>>,
}

// Main struct holding all sensor function structs
pub struct HayasenFunctions<I2C, E> {
    #[cfg(feature = "mpu9250")]
    pub mpu9250: MPU9250Functions<I2C, E>,
}

impl<I2C, E> HayasenFunctions<I2C, E> 
where 
    I2C: I2c<Error = E>
{
    pub fn new() -> Self {
        HayasenFunctions {
            #[cfg(feature = "mpu9250")]
            mpu9250 : MPU9250Functions {
                        verify_identity: mpu9250::Mpu9250::verify_identity,
                        configure_power : mpu9250::Mpu9250::configure_power,
                        setup_accelerometer : mpu9250::Mpu9250::setup_accelerometer,
                        setup_gyroscope : mpu9250::Mpu9250::setup_gyroscope,
                        initialize_sensor : mpu9250::Mpu9250::initialize_sensor,
                        read_accel_raw : mpu9250::Mpu9250::read_accel_raw,
                        read_gyro_raw : mpu9250::Mpu9250::read_gyro_raw,
                        read_temp_raw : mpu9250::Mpu9250::read_temp_raw,
                        read_acceleration : mpu9250::Mpu9250::read_acceleration,
                        read_angular_velocity : mpu9250::Mpu9250::read_angular_velocity,
                        read_temperature_celsius : mpu9250::Mpu9250::read_temperature_celsius,
                        set_sample_rate : mpu9250::Mpu9250::set_sample_rate,
                        set_dlpf_config : mpu9250::Mpu9250::set_dlpf_config,
                        enter_sleep_mode : mpu9250::Mpu9250::enter_sleep_mode,
                        wake_up : mpu9250::Mpu9250::wake_up,
            }
        }
    }    
}

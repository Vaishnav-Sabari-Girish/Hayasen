#[cfg(feature = "mpu9250")]
pub mod mpu9250 {
    pub use crate::mpu9250::*;
}

#[cfg(feature = "mpu6050")]
pub mod mpu6050 {
    pub use crate::mpu6050::*;
}

#[cfg(feature = "max30102")]
pub mod max30102 {
    pub use crate::max30102::*;
}

use embedded_hal::i2c::I2c;
use crate::error::Error;

#[cfg(feature = "mpu9250")]
#[cfg_attr(docsrs, doc(cfg(feature = "mpu9250")))]
pub struct MPU9250Functions<I2C, E> {
    pub verify_identity: fn(&mut mpu9250::Mpu9250<I2C>) -> Result<(), Error<E>>,
    pub configure_power: fn(&mut mpu9250::Mpu9250<I2C>) -> Result<(), Error<E>>,
    pub setup_accelerometer: fn(&mut mpu9250::Mpu9250<I2C>, mpu9250::AccelRange) -> Result<(), Error<E>>,
    pub setup_gyroscope: fn(&mut mpu9250::Mpu9250<I2C>, mpu9250::GyroRange) -> Result<(), Error<E>>,
    pub initialize_sensor: fn(&mut mpu9250::Mpu9250<I2C>, mpu9250::AccelRange, mpu9250::GyroRange) -> Result<(), Error<E>>,
    pub read_accel_raw: fn(&mut mpu9250::Mpu9250<I2C>) -> Result<[i16; 3], Error<E>>,
    pub read_gyro_raw: fn(&mut mpu9250::Mpu9250<I2C>) -> Result<[i16; 3], Error<E>>,
    pub read_temp_raw: fn(&mut mpu9250::Mpu9250<I2C>) -> Result<i16, Error<E>>,
    pub read_acceleration: fn(&mut mpu9250::Mpu9250<I2C>) -> Result<[f32; 3], Error<E>>,
    pub read_angular_velocity: fn(&mut mpu9250::Mpu9250<I2C>) -> Result<[f32; 3], Error<E>>,
    pub read_temperature_celsius: fn(&mut mpu9250::Mpu9250<I2C>) -> Result<f32, Error<E>>,
    pub set_sample_rate: fn(&mut mpu9250::Mpu9250<I2C>, u8) -> Result<(), Error<E>>,
    pub set_dlpf_config: fn(&mut mpu9250::Mpu9250<I2C>, mpu9250::DlpfConfig) -> Result<(), Error<E>>,
    pub enter_sleep_mode: fn(&mut mpu9250::Mpu9250<I2C>) -> Result<(), Error<E>>,
    pub wake_up: fn(&mut mpu9250::Mpu9250<I2C>) -> Result<(), Error<E>>,
}

#[cfg(feature = "mpu6050")]
#[cfg_attr(docsrs, doc(cfg(feature = "mpu6050")))]
pub struct MPU6050Functions<I2C, E> {
    pub verify_identity: fn(&mut mpu6050::Mpu6050<I2C>) -> Result<(), Error<E>>,
    pub configure_power: fn(&mut mpu6050::Mpu6050<I2C>) -> Result<(), Error<E>>,
    pub setup_accelerometer: fn(&mut mpu6050::Mpu6050<I2C>, mpu6050::AccelRange) -> Result<(), Error<E>>,
    pub setup_gyroscope: fn(&mut mpu6050::Mpu6050<I2C>, mpu6050::GyroRange) -> Result<(), Error<E>>,
    pub initialize_sensor: fn(&mut mpu6050::Mpu6050<I2C>, mpu6050::AccelRange, mpu6050::GyroRange) -> Result<(), Error<E>>,
    pub read_accel_raw: fn(&mut mpu6050::Mpu6050<I2C>) -> Result<[i16; 3], Error<E>>,
    pub read_gyro_raw: fn(&mut mpu6050::Mpu6050<I2C>) -> Result<[i16; 3], Error<E>>,
    pub read_temp_raw: fn(&mut mpu6050::Mpu6050<I2C>) -> Result<i16, Error<E>>,
    pub read_acceleration: fn(&mut mpu6050::Mpu6050<I2C>) -> Result<[f32; 3], Error<E>>,
    pub read_angular_velocity: fn(&mut mpu6050::Mpu6050<I2C>) -> Result<[f32; 3], Error<E>>,
    pub read_temperature_celsius: fn(&mut mpu6050::Mpu6050<I2C>) -> Result<f32, Error<E>>,
    pub set_sample_rate: fn(&mut mpu6050::Mpu6050<I2C>, u8) -> Result<(), Error<E>>,
    pub set_dlpf_config: fn(&mut mpu6050::Mpu6050<I2C>, mpu6050::DlpfConfig) -> Result<(), Error<E>>,
    pub enter_sleep_mode: fn(&mut mpu6050::Mpu6050<I2C>) -> Result<(), Error<E>>,
    pub wake_up: fn(&mut mpu6050::Mpu6050<I2C>) -> Result<(), Error<E>>,
    pub disable_sleep: fn(&mut mpu6050::Mpu6050<I2C>) -> Result<(), Error<E>>,
    pub enable_temperature_sensor: fn(&mut mpu6050::Mpu6050<I2C>) -> Result<(), Error<E>>,
    pub disable_temperature_sensor: fn(&mut mpu6050::Mpu6050<I2C>) -> Result<(), Error<E>>,
}

#[cfg(feature = "max30102")]
#[cfg_attr(docsrs, doc(cfg(feature = "max30102")))]
pub struct MAX30102Functions<I2C, E> {
    // Core sensor operations
    pub verify_identity: fn(&mut max30102::Max30102<I2C>) -> Result<(), Error<E>>,
    pub reset: fn(&mut max30102::Max30102<I2C>) -> Result<(), Error<E>>,
    pub shutdown: fn(&mut max30102::Max30102<I2C>) -> Result<(), Error<E>>,
    pub wakeup: fn(&mut max30102::Max30102<I2C>) -> Result<(), Error<E>>,
    pub force_reset: fn(&mut max30102::Max30102<I2C>) -> Result<(), Error<E>>,
    
    // Configuration methods
    pub set_operation_mode: fn(&mut max30102::Max30102<I2C>, max30102::OperationMode) -> Result<(), Error<E>>,
    pub set_adc_range: fn(&mut max30102::Max30102<I2C>, max30102::AdcRange) -> Result<(), Error<E>>,
    pub set_sampling_rate: fn(&mut max30102::Max30102<I2C>, max30102::SamplingRate) -> Result<(), Error<E>>,
    pub set_pulse_width: fn(&mut max30102::Max30102<I2C>, max30102::LedPulseWidth) -> Result<(), Error<E>>,
    pub set_sample_averaging: fn(&mut max30102::Max30102<I2C>, max30102::SampleAveraging) -> Result<(), Error<E>>,
    
    // FIFO configuration
    pub enable_fifo_rollover: fn(&mut max30102::Max30102<I2C>, bool) -> Result<(), Error<E>>,
    pub set_fifo_almost_full_threshold: fn(&mut max30102::Max30102<I2C>, u8) -> Result<(), Error<E>>,
    pub clear_fifo: fn(&mut max30102::Max30102<I2C>) -> Result<(), Error<E>>,
    
    // LED configuration
    pub set_led_pulse_amplitude: fn(&mut max30102::Max30102<I2C>, u8, u8) -> Result<(), Error<E>>,
    pub set_pilot_led_amplitude: fn(&mut max30102::Max30102<I2C>, u8) -> Result<(), Error<E>>,
    pub set_multi_led_slots: fn(&mut max30102::Max30102<I2C>, max30102::LedSlot, max30102::LedSlot, max30102::LedSlot, max30102::LedSlot) -> Result<(), Error<E>>,
    
    // Interrupt management
    pub enable_interrupt: fn(&mut max30102::Max30102<I2C>, max30102::InterruptSource) -> Result<(), Error<E>>,
    pub disable_interrupt: fn(&mut max30102::Max30102<I2C>, max30102::InterruptSource) -> Result<(), Error<E>>,
    pub read_interrupt_status: fn(&mut max30102::Max30102<I2C>) -> Result<(u8, u8), Error<E>>,
    
    // Data reading
    pub get_available_sample_count: fn(&mut max30102::Max30102<I2C>) -> Result<u8, Error<E>>,
    pub read_fifo_sample: fn(&mut max30102::Max30102<I2C>) -> Result<Option<max30102::FifoSample>, Error<E>>,
    pub read_fifo_batch: fn(&mut max30102::Max30102<I2C>, &mut [max30102::FifoSample]) -> Result<usize, Error<E>>,
    
    // Temperature measurement
    pub start_temperature_measurement: fn(&mut max30102::Max30102<I2C>) -> Result<(), Error<E>>,
    pub read_temperature: fn(&mut max30102::Max30102<I2C>) -> Result<Option<f32>, Error<E>>,
    
    // Proximity detection
    pub set_proximity_threshold: fn(&mut max30102::Max30102<I2C>, u8) -> Result<(), Error<E>>,
    
    // Initialization and status
    pub initialize_sensor: fn(&mut max30102::Max30102<I2C>) -> Result<(), Error<E>>,
    pub initialize_heart_rate_mode: fn(&mut max30102::Max30102<I2C>) -> Result<(), Error<E>>,
    pub get_adc_resolution: fn(&mut max30102::Max30102<I2C>) -> Result<u8, Error<E>>,
    pub is_shutdown: fn(&mut max30102::Max30102<I2C>) -> Result<bool, Error<E>>,
    pub get_operation_mode: fn(&mut max30102::Max30102<I2C>) -> Result<max30102::OperationMode, Error<E>>,
    pub validate_configuration: fn(&mut max30102::Max30102<I2C>) -> Result<(), Error<E>>,
}

pub struct HayasenFunctions<I2C, E> {
    #[cfg(feature = "mpu9250")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mpu9250")))]
    pub mpu9250: MPU9250Functions<I2C, E>,
    
    #[cfg(feature = "mpu6050")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mpu6050")))]
    pub mpu6050: MPU6050Functions<I2C, E>,
    
    #[cfg(feature = "max30102")]
    #[cfg_attr(docsrs, doc(cfg(feature = "max30102")))]
    pub max30102: MAX30102Functions<I2C, E>,
}

impl<I2C, E> HayasenFunctions<I2C, E> 
where 
    I2C: I2c<Error = E>
{
    pub fn new() -> Self {
        HayasenFunctions {
            #[cfg(feature = "mpu9250")]
            mpu9250: MPU9250Functions {
                verify_identity: mpu9250::Mpu9250::verify_identity,
                configure_power: mpu9250::Mpu9250::configure_power,
                setup_accelerometer: mpu9250::Mpu9250::setup_accelerometer,
                setup_gyroscope: mpu9250::Mpu9250::setup_gyroscope,
                initialize_sensor: mpu9250::Mpu9250::initialize_sensor,
                read_accel_raw: mpu9250::Mpu9250::read_accel_raw,
                read_gyro_raw: mpu9250::Mpu9250::read_gyro_raw,
                read_temp_raw: mpu9250::Mpu9250::read_temp_raw,
                read_acceleration: mpu9250::Mpu9250::read_acceleration,
                read_angular_velocity: mpu9250::Mpu9250::read_angular_velocity,
                read_temperature_celsius: mpu9250::Mpu9250::read_temperature_celsius,
                set_sample_rate: mpu9250::Mpu9250::set_sample_rate,
                set_dlpf_config: mpu9250::Mpu9250::set_dlpf_config,
                enter_sleep_mode: mpu9250::Mpu9250::enter_sleep_mode,
                wake_up: mpu9250::Mpu9250::wake_up,
            },
            
            #[cfg(feature = "mpu6050")]
            mpu6050: MPU6050Functions {
                verify_identity: mpu6050::Mpu6050::verify_identity,
                configure_power: mpu6050::Mpu6050::configure_power,
                setup_accelerometer: mpu6050::Mpu6050::setup_accelerometer,
                setup_gyroscope: mpu6050::Mpu6050::setup_gyroscope,
                initialize_sensor: mpu6050::Mpu6050::initialize_sensor,
                read_accel_raw: mpu6050::Mpu6050::read_accel_raw,
                read_gyro_raw: mpu6050::Mpu6050::read_gyro_raw,
                read_temp_raw: mpu6050::Mpu6050::read_temp_raw,
                read_acceleration: mpu6050::Mpu6050::read_acceleration,
                read_angular_velocity: mpu6050::Mpu6050::read_angular_velocity,
                read_temperature_celsius: mpu6050::Mpu6050::read_temperature_celsius,
                set_sample_rate: mpu6050::Mpu6050::set_sample_rate,
                set_dlpf_config: mpu6050::Mpu6050::set_dlpf_config,
                enter_sleep_mode: mpu6050::Mpu6050::enter_sleep_mode,
                wake_up: mpu6050::Mpu6050::wake_up,
                disable_sleep: mpu6050::Mpu6050::disable_sleep,
                enable_temperature_sensor: mpu6050::Mpu6050::enable_temperature_sensor,
                disable_temperature_sensor: mpu6050::Mpu6050::disable_temperature_sensor,
            },
            
            #[cfg(feature = "max30102")]
            max30102: MAX30102Functions {
                // Core sensor operations
                verify_identity: max30102::Max30102::verify_identity,
                reset: max30102::Max30102::reset,
                shutdown: max30102::Max30102::shutdown,
                wakeup: max30102::Max30102::wakeup,
                force_reset: max30102::Max30102::force_reset,
                
                // Configuration methods
                set_operation_mode: max30102::Max30102::set_operation_mode,
                set_adc_range: max30102::Max30102::set_adc_range,
                set_sampling_rate: max30102::Max30102::set_sampling_rate,
                set_pulse_width: max30102::Max30102::set_pulse_width,
                set_sample_averaging: max30102::Max30102::set_sample_averaging,
                
                // FIFO configuration
                enable_fifo_rollover: max30102::Max30102::enable_fifo_rollover,
                set_fifo_almost_full_threshold: max30102::Max30102::set_fifo_almost_full_threshold,
                clear_fifo: max30102::Max30102::clear_fifo,
                
                // LED configuration
                set_led_pulse_amplitude: max30102::Max30102::set_led_pulse_amplitude,
                set_pilot_led_amplitude: max30102::Max30102::set_pilot_led_amplitude,
                set_multi_led_slots: max30102::Max30102::set_multi_led_slots,
                
                // Interrupt management
                enable_interrupt: max30102::Max30102::enable_interrupt,
                disable_interrupt: max30102::Max30102::disable_interrupt,
                read_interrupt_status: max30102::Max30102::read_interrupt_status,
                
                // Data reading
                get_available_sample_count: max30102::Max30102::get_available_sample_count,
                read_fifo_sample: max30102::Max30102::read_fifo_sample,
                read_fifo_batch: max30102::Max30102::read_fifo_batch,
                
                // Temperature measurement
                start_temperature_measurement: max30102::Max30102::start_temperature_measurement,
                read_temperature: max30102::Max30102::read_temperature,
                
                // Proximity detection
                set_proximity_threshold: max30102::Max30102::set_proximity_threshold,
                
                // Initialization and status
                initialize_sensor: max30102::Max30102::initialize_sensor,
                initialize_heart_rate_mode: max30102::Max30102::initialize_heart_rate_mode,
                get_adc_resolution: max30102::Max30102::get_adc_resolution,
                is_shutdown: max30102::Max30102::is_shutdown,
                get_operation_mode: max30102::Max30102::get_operation_mode,
                validate_configuration: max30102::Max30102::validate_configuration,
            }
        }
    }
}

impl<I2C, E> Default for HayasenFunctions<I2C, E>
where
    I2C: I2c<Error = E>,
{
    fn default() -> Self {
        Self::new()
    }
}

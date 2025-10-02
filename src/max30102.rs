#[cfg(feature = "max30102")]
use embedded_hal::i2c::I2c;

#[cfg(feature = "max30102")]
use crate::error::Error;

#[cfg(feature = "max30102")]
mod registers {
    // Device Identification
    pub const PART_ID: u8 = 0xFF;

    // Status Registers
    pub const INT_STATUS_1: u8 = 0x00;
    pub const INT_STATUS_2: u8 = 0x01;

    // Interrupt enable registers
    pub const INT_ENABLE_1: u8 = 0x02;
    pub const INT_ENABLE_2: u8 = 0x03;

    // FIFO registers
    pub const FIFO_WR_PTR: u8 = 0x04;
    pub const FIFO_OVF_CNT: u8 = 0x05;
    pub const FIFO_RD_PTR: u8 = 0x06;
    pub const FIFO_DATA: u8 = 0x07;

    // Configuration Registers
    pub const FIFO_CONFIG: u8 = 0x08;
    pub const MODE_CONFIG: u8 = 0x09;
    pub const SPO2_CONFIG: u8 = 0x0A;

    // LED Pulse amplitude registers
    pub const LED1_PA: u8 = 0x0C;
    pub const LED2_PA: u8 = 0x0D;
    pub const PILOT_PA: u8 = 0x10;

    // Multi-LED Mode Configuration
    pub const MULTI_LED_CONFIG1: u8 = 0x11;
    pub const MULTI_LED_CONFIG2: u8 = 0x12;

    // Temperature Registers
    pub const TEMP_INTR: u8 = 0x1F;
    pub const TEMP_FRAC: u8 = 0x20;
    pub const TEMP_CONFIG: u8 = 0x21;

    // Proximity Interrupt threshold
    pub const PROX_INT_THRESH: u8 = 0x30;
}

#[cfg(feature = "max30102")]
use registers::*;

#[cfg(feature = "max30102")]
pub struct Max30102<I2C> {
    i2c: I2C,
    address: u8
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg(feature = "max30102")]
pub enum OperationMode {
    HeartRate = 0x02,
    SpO2 = 0x03,
    MultiLed = 0x07,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg(feature = "max30102")]
pub enum SamplingRate {
    Rate50 = 0x00,
    Rate100 = 0x01,
    Rate200 = 0x02,
    Rate400 = 0x03,
    Rate800 = 0x04,
    Rate1000 = 0x05,
    Rate1600 = 0x06,
    Rate3200 = 0x07,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg(feature = "max30102")]
pub enum LedPulseWidth {
    Width69us = 0x00,    // 15-bit resolution
    Width118us = 0x01,   // 16-bit resolution
    Width215us = 0x02,   // 17-bit resolution
    Width411us = 0x03,   // 18-bit resolution
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg(feature = "max30102")]
pub enum AdcRange {
    Range2048na = 0x00,   // LSB = 7.81 pA
    Range4096na = 0x01,   // LSB = 15.63 pA
    Range8192na = 0x02,   // LSB = 31.25 pA
    Range16384na = 0x03,  // LSB = 62.5 pA
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg(feature = "max30102")]
pub enum SampleAveraging {
    NoAveraging = 0x00,
    Average2 = 0x01,
    Average4 = 0x02,
    Average8 = 0x03,
    Average16 = 0x04,
    Average32 = 0x05,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg(feature = "max30102")]
pub enum InterruptSource {
    FifoAlmostFull = 0x80,    // Bit 7 of INT_ENABLE_1
    NewDataReady = 0x40,      // Bit 6 of INT_ENABLE_1
    AlcOverflow = 0x20,       // Bit 5 of INT_ENABLE_1
    PowerReady = 0x01,        // Bit 0 of INT_ENABLE_1
    TemperatureReady = 0x02,  // Bit 1 of INT_ENABLE_2
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg(feature = "max30102")]
pub enum LedSlot {
    None = 0x00, 
    Led1Red = 0x01, 
    Led2Ir = 0x02, 
    Led3Green = 0x03,    // For MAX30101 only 
    PilotLed1 = 0x05,
    PilotLed2 = 0x06,
    PilotLed3 = 0x07,   // For MAX30101 only
}

#[derive(Debug)]
#[cfg(feature = "max30102")]
pub struct FifoSample {
    pub red: u32,
    pub ir: u32,
}

#[cfg(feature = "max30102")]
impl<I2C, E>  Max30102<I2C>
where 
    I2C: I2c<Error = E>
{
    // Default I2C address for MAX30102
    pub const DEFAULT_ADDRESS: u8 = 0x57;

    // Maximum FIFO Depth
    pub const FIFO_DEPTH: u8 = 32;

    // Expected Part ID for MAX30102
    pub const EXPECTED_PART_ID: u8 = 0x15;

    pub fn new(i2c: I2C, address: u8) -> Self {
        Max30102 { i2c, address }
    }

    // Create a new instance with default I2C address
    pub fn new_default(i2c: I2C) -> Self {
        Self::new(i2c, Self::DEFAULT_ADDRESS)
    }

    pub fn verify_identity(&mut self) -> Result<(), Error<E>> {
        let mut buffer = [0u8];
        self.i2c.write_read(self.address, &[PART_ID], &mut buffer)?;

        match buffer[0] {
            Self::EXPECTED_PART_ID => Ok(()),
            _ => Err(Error::NotDetected)
        }
    }

    pub fn reset(&mut self) -> Result<(), Error<E>> {
        self.i2c.write(self.address, &[MODE_CONFIG, 0x40])?;
        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<(), Error<E>> {
        // Set shutdown bit (Bit 7) in MODE_CONFIG register
        let mut current_config = [0u8];
        self.i2c.write_read(self.address, &[MODE_CONFIG], &mut current_config)?;
        let new_config = current_config[0] | 0x80;
        self.i2c.write(self.address, &[MODE_CONFIG, new_config])?;
        Ok(())
    }

    pub fn wakeup(&mut self) -> Result<(), Error<E>> {
        // Clear shutdown bit in MODE_CONFIG register
        let mut current_config = [0u8];
        self.i2c.write_read(self.address, &[MODE_CONFIG], &mut current_config)?;
        let new_config = current_config[0] & 0x7F;  // Fixed: should be & not |
        self.i2c.write(self.address, &[MODE_CONFIG, new_config])?;
        Ok(())
    }

    pub fn set_operation_mode(&mut self, mode: OperationMode) -> Result<(), Error<E>> {
        let mut current_config = [0u8];
        self.i2c.write_read(self.address, &[MODE_CONFIG], &mut current_config)?;

        // Clear mode bits and set new mode (preserve other bits)
        let new_config = (current_config[0] & 0xF8) | (mode as u8);
        self.i2c.write(self.address, &[MODE_CONFIG, new_config])?;
        Ok(())
    }

    pub fn set_adc_range(&mut self, range: AdcRange) -> Result<(), Error<E>> {
        let mut current_config = [0u8];
        self.i2c.write_read(self.address, &[SPO2_CONFIG], &mut current_config)?;

        // Clear ADC bits (Bits 6:5) and set new Range 
        let new_config = (current_config[0] & 0x9F) | ((range as u8) << 5);
        self.i2c.write(self.address, &[SPO2_CONFIG, new_config])?;
        Ok(())
    }

    pub fn set_sampling_rate(&mut self, rate: SamplingRate) -> Result<(), Error<E>> {
        let mut current_config = [0u8];
        self.i2c.write_read(self.address, &[SPO2_CONFIG], &mut current_config)?;

        // Clear sampling rate bits (Bits 4:2) and set new Rate
        let new_config = (current_config[0] & 0xE3) | ((rate as u8) << 2);
        self.i2c.write(self.address, &[SPO2_CONFIG, new_config])?;
        Ok(())
    }

    pub fn set_pulse_width(&mut self, width: LedPulseWidth) -> Result<(), Error<E>> {
        let mut current_config = [0u8];
        self.i2c.write_read(self.address, &[SPO2_CONFIG], &mut current_config)?;

        // Clear pulse width bits (Bits 1:0) and set new width
        let new_config = (current_config[0] & 0xFC) | (width as u8);
        self.i2c.write(self.address, &[SPO2_CONFIG, new_config])?;
        Ok(())
    }

    pub fn set_sample_averaging(&mut self, averaging: SampleAveraging) -> Result<(), Error<E>> {
        let mut current_config = [0u8];
        self.i2c.write_read(self.address, &[FIFO_CONFIG], &mut current_config)?;

        // Clear Sample Averaging bits (bits 7:5) and set new averaging
        let new_config = (current_config[0] & 0x1F) | ((averaging as u8) << 5);
        self.i2c.write(self.address, &[FIFO_CONFIG, new_config])?;
        Ok(())
    }

    pub fn enable_fifo_rollover(&mut self, enable: bool) -> Result<(), Error<E>> {
        let mut current_config = [0u8];
        self.i2c.write_read(self.address, &[FIFO_CONFIG], &mut current_config)?;

        let new_config = if enable {
            current_config[0] | 0x10    // Set bit 4
        } else {
            current_config[0] & 0xEF    // Clear Bit 4
        };

        self.i2c.write(self.address, &[FIFO_CONFIG, new_config])?;
        Ok(())
    }

    pub fn set_fifo_almost_full_threshold(&mut self, threshold: u8) -> Result<(), Error<E>> {
        if threshold > 15 {
            return Err(Error::ConfigError);
        }

        let mut current_config = [0u8];
        self.i2c.write_read(self.address, &[FIFO_CONFIG], &mut current_config)?;

        // Clear FIFO_A_FULL bits (bits 3:0) and set new threshold
        let new_config = (current_config[0] & 0xF0) | threshold;
        self.i2c.write(self.address, &[FIFO_CONFIG, new_config])?;
        Ok(())
    }

    pub fn set_led_pulse_amplitude(&mut self, led: u8, amplitude: u8) -> Result<(), Error<E>> {
        let register = match led {
            1 => LED1_PA,
            2 => LED2_PA,
            _ => return Err(Error::ConfigError),
        };

        self.i2c.write(self.address, &[register, amplitude])?;
        Ok(())
    }

    pub fn set_pilot_led_amplitude(&mut self, amplitude: u8) -> Result<(), Error<E>> {
        self.i2c.write(self.address, &[PILOT_PA, amplitude])?;
        Ok(())
    }

    pub fn enable_interrupt(&mut self, interrupt: InterruptSource) -> Result<(), Error<E>> {
        let (register, mask) = match interrupt {
            InterruptSource::FifoAlmostFull => (INT_ENABLE_1, 0x80),
            InterruptSource::NewDataReady => (INT_ENABLE_1, 0x40),
            InterruptSource::AlcOverflow => (INT_ENABLE_1, 0x20),
            InterruptSource::PowerReady => (INT_ENABLE_1, 0x01),
            InterruptSource::TemperatureReady => (INT_ENABLE_2, 0x02),
        };

        let mut current_config = [0u8];
        self.i2c.write_read(self.address, &[register], &mut current_config)?;

        let new_config = current_config[0] | mask;
        self.i2c.write(self.address, &[register, new_config])?;
        Ok(())
    }

    pub fn disable_interrupt(&mut self, interrupt: InterruptSource) -> Result<(), Error<E>> {
        let (register, mask) = match interrupt {
            InterruptSource::FifoAlmostFull => (INT_ENABLE_1, 0x80),
            InterruptSource::NewDataReady => (INT_ENABLE_1, 0x40),
            InterruptSource::AlcOverflow => (INT_ENABLE_1, 0x20),
            InterruptSource::PowerReady => (INT_ENABLE_1, 0x01),
            InterruptSource::TemperatureReady => (INT_ENABLE_2, 0x02),
        };

        let mut current_config = [0u8];
        self.i2c.write_read(self.address, &[register], &mut current_config)?;

        let new_config = current_config[0] & !mask;
        self.i2c.write(self.address, &[register, new_config])?;
        Ok(())
    }

    pub fn read_interrupt_status(&mut self) -> Result<(u8, u8), Error<E>> {
        let mut status1 = [0u8];
        let mut status2 = [0u8];

        self.i2c.write_read(self.address, &[INT_STATUS_1], &mut status1)?;
        self.i2c.write_read(self.address, &[INT_STATUS_2], &mut status2)?;

        Ok((status1[0], status2[0]))
    }

    pub fn get_available_sample_count(&mut self) -> Result<u8, Error<E>> {
        let mut wr_ptr = [0u8];
        let mut rd_ptr = [0u8];

        self.i2c.write_read(self.address, &[FIFO_WR_PTR], &mut wr_ptr)?;
        self.i2c.write_read(self.address, &[FIFO_RD_PTR], &mut rd_ptr)?;

        // Handle 5-bit wraparound correctly
        let wr = wr_ptr[0] & 0x1F;
        let rd = rd_ptr[0] & 0x1F;

        let count = if wr >= rd {
            wr - rd
        } else {
            Self::FIFO_DEPTH - rd + wr
        };

        Ok(count)
    }

    pub fn read_fifo_sample(&mut self) -> Result<Option<FifoSample>, Error<E>> {
        let available = self.get_available_sample_count()?;

        if available == 0 {
            return Ok(None);
        }

        // Read 6 bytes for SpO2 mode (3 bytes IR + 3 bytes Red)
        let mut buffer = [0u8; 6];
        self.i2c.write_read(self.address, &[FIFO_DATA], &mut buffer)?;

        // Parse the data with proper 18-bit masking
        let ir = (((buffer[0] as u32) << 16) | 
            ((buffer[1] as u32) << 8) | 
            (buffer[2] as u32)) & 0x03FFFF;   // Mask to 18-bits

        let red = (((buffer[3] as u32) << 16) |
            ((buffer[4] as u32) << 8) |
            (buffer[5] as u32)) & 0x03FFFF;  // Mask to 18-bits

        Ok(Some(FifoSample { red, ir }))
    }

    pub fn read_fifo_batch(&mut self, samples: &mut [FifoSample]) -> Result<usize, Error<E>> {
        let available = self.get_available_sample_count()? as usize;
        let to_read = available.min(samples.len());

        if to_read == 0 {
            return Ok(0);
        }

        // Use a fixed-size buffer instead of vec!
        // MAX30102 FIFO holds max 32 samples, so 32 * 6 = 192 bytes max
        const MAX_BUFFER_SIZE: usize = 192;
        let mut buffer = [0u8; MAX_BUFFER_SIZE];
        
        let bytes_to_read = to_read * 6;  // 6 bytes per sample in SpO2 mode
        
        if bytes_to_read > MAX_BUFFER_SIZE {
            return Err(Error::ConfigError);
        }

        // Read the data
        self.i2c.write_read(self.address, &[FIFO_DATA], &mut buffer[..bytes_to_read])?;

        // Parse the samples
        for (i, sample) in samples[..to_read].iter_mut().enumerate() {
            let offset = i * 6;

            sample.ir = (((buffer[offset] as u32) << 16) | 
                ((buffer[offset + 1] as u32) << 8) | 
                (buffer[offset + 2] as u32)) & 0x03FFFF;

            sample.red = (((buffer[offset + 3] as u32) << 16) | 
                ((buffer[offset + 4] as u32) << 8) | 
                (buffer[offset + 5] as u32)) & 0x03FFFF;
        }

        Ok(to_read)
    }

    pub fn clear_fifo(&mut self) -> Result<(), Error<E>> {
        // Reset FIFO read and write pointers
        self.i2c.write(self.address, &[FIFO_WR_PTR, 0x00])?;
        self.i2c.write(self.address, &[FIFO_OVF_CNT, 0x00])?;
        self.i2c.write(self.address, &[FIFO_RD_PTR, 0x00])?;
        Ok(())
    }

    pub fn start_temperature_measurement(&mut self) -> Result<(), Error<E>> {
        // Enable temperature ready interrupt for non-blocking operation
        self.enable_interrupt(InterruptSource::TemperatureReady)?;

        // Start temperature measurement by writing 0x01 to TEMP_CONFIG
        self.i2c.write(self.address, &[TEMP_CONFIG, 0x01])?;
        Ok(())
    }

    pub fn read_temperature(&mut self) -> Result<Option<f32>, Error<E>> {
        // Check if temperature measurement is ready
        let (_, status2) = self.read_interrupt_status()?;

        if (status2 & 0x02) == 0 {
            return Ok(None);
        }

        let mut temp_int = [0u8];
        let mut temp_frac = [0u8];

        self.i2c.write_read(self.address, &[TEMP_INTR], &mut temp_int)?;
        self.i2c.write_read(self.address, &[TEMP_FRAC], &mut temp_frac)?;

        // Temperature calculation 
        let integer = temp_int[0] as i8 as f32;
        let fraction = (temp_frac[0] & 0x0F) as f32 * 0.0625;

        Ok(Some(integer + fraction))
    }

    pub fn set_multi_led_slots(&mut self, slot1: LedSlot, slot2: LedSlot, slot3: LedSlot, slot4: LedSlot) -> Result<(), Error<E>> {
        let config1 = ((slot2 as u8) << 4) | (slot1 as u8);
        let config2 = ((slot4 as u8) << 4) | (slot3 as u8);

        self.i2c.write(self.address, &[MULTI_LED_CONFIG1, config1])?;
        self.i2c.write(self.address, &[MULTI_LED_CONFIG2, config2])?;
        Ok(())
    }

    pub fn set_proximity_threshold(&mut self, threshold: u8) -> Result<(), Error<E>> {
        self.i2c.write(self.address, &[PROX_INT_THRESH, threshold])?;
        Ok(())
    }

    pub fn initialize_sensor(&mut self) -> Result<(), Error<E>> {
        // Verify sensor identity
        self.verify_identity()?;

        // Reset the sensor
        self.reset()?;

        // Clear FIFO
        self.clear_fifo()?;

        // Configure for SpO2 mode with optimal settings
        self.set_operation_mode(OperationMode::SpO2)?;

        // Set ADC range to 4096nA for good dynamic range
        self.set_adc_range(AdcRange::Range4096na)?;

        // Set sampling rate to 100 samples per second
        self.set_sampling_rate(SamplingRate::Rate100)?;

        // Set pulse width to 411us for maximum resolution
        self.set_pulse_width(LedPulseWidth::Width411us)?;

        // Enable Sample Averaging (4 samples) to reduce noise
        self.set_sample_averaging(SampleAveraging::Average4)?;

        // Enable FIFO rollover to prevent data loss
        self.enable_fifo_rollover(true)?;

        // Set FIFO to almost full threshold to trigger when 15 free spaces remain
        self.set_fifo_almost_full_threshold(15)?;

        // Set reasonable LED pulse amplitude (Adjustable based on requirements)
        self.set_led_pulse_amplitude(1, 0x1F)?;   // Red LED
        self.set_led_pulse_amplitude(2, 0x1F)?;   // IR LED

        // Enable FIFO almost full interrupt
        self.enable_interrupt(InterruptSource::FifoAlmostFull)?;

        // Enable new data ready interrupt for real-time processing
        self.enable_interrupt(InterruptSource::NewDataReady)?;

        Ok(())
    }

    // Heart rate only mode sensor initialization
    pub fn initialize_heart_rate_mode(&mut self) -> Result<(), Error<E>> {
        self.verify_identity()?;
        self.reset()?;
        self.clear_fifo()?;

        // Configure for HeartRate mode
        self.set_operation_mode(OperationMode::HeartRate)?;
        self.set_adc_range(AdcRange::Range4096na)?;
        self.set_sampling_rate(SamplingRate::Rate100)?;
        self.set_pulse_width(LedPulseWidth::Width411us)?;
        self.set_sample_averaging(SampleAveraging::Average4)?;

        // Only IR LED is used in HeartRate mode
        self.set_led_pulse_amplitude(2, 0x1F)?;

        self.enable_interrupt(InterruptSource::FifoAlmostFull)?;
        Ok(())
    }

    // Get the resolution in bits based on current pulse width scaling
    pub fn get_adc_resolution(&mut self) -> Result<u8, Error<E>> {
        let mut config = [0u8];
        self.i2c.write_read(self.address, &[SPO2_CONFIG], &mut config)?;

        let pulse_width = config[0] & 0x03;
        let resolution = match pulse_width {
            0 => 15,      // 69us pulse width
            1 => 16,      // 118us pulse width
            2 => 17,      // 215us pulse width
            3 => 18,      // 411us pulse width
            _ => return Err(Error::InvalidData),
        };

        Ok(resolution)
    }

    // Check if sensor is in shutdown mode
    pub fn is_shutdown(&mut self) -> Result<bool, Error<E>> {
        let mut config = [0u8];
        self.i2c.write_read(self.address, &[MODE_CONFIG], &mut config)?;
        Ok((config[0] & 0x80) != 0)
    }

    // Get current operation mode
    pub fn get_operation_mode(&mut self) -> Result<OperationMode, Error<E>> {
        let mut config = [0u8];
        self.i2c.write_read(self.address, &[MODE_CONFIG], &mut config)?;

        match config[0] & 0x07 {
            0x02 => Ok(OperationMode::HeartRate),
            0x03 => Ok(OperationMode::SpO2),
            0x07 => Ok(OperationMode::MultiLed),
            _ => Err(Error::InvalidData),
        }
    }

    pub fn validate_configuration(&mut self) -> Result<(), Error<E>> {
        // Check if sensor is detected
        self.verify_identity()?;

        // Check if sensor is not in shutdown
        if self.is_shutdown()? {
            return Err(Error::SensorSpecific("Sensor is in Shutdown Mode"));
        }

        let _mode = self.get_operation_mode()?;

        // Check FIFO is not overflowing
        let mut overflow_count = [0u8];
        self.i2c.write_read(self.address, &[FIFO_OVF_CNT], &mut overflow_count)?;

        if overflow_count[0] > 0 {
            return Err(Error::SensorSpecific("FIFO overflow detected"));
        }

        Ok(())
    }

    // Reset sensor to known good state
    pub fn force_reset(&mut self) -> Result<(), Error<E>> {
        // Perform hard reset sequence
        self.reset()?;

        let mut attempts = 0;
        const MAX_ATTEMPTS: u8 = 3;

        while attempts < MAX_ATTEMPTS {
            match self.verify_identity() {
                Ok(()) => break,
                Err(Error::NotDetected) if attempts < MAX_ATTEMPTS - 1 => {
                    attempts += 1;
                    continue;
                }
                Err(e) => return Err(e),
            }
        }

        if attempts >= MAX_ATTEMPTS {
            return Err(Error::SensorSpecific("Reset Failed: Sensor not responding"));
        }

        Ok(())
    }
}

#[cfg(feature = "mpu9250")]

use embedded_hal::i2c::I2c;
use crate::error::Error;

//Register addresses 
const WHO_AM_I: u8 = 0x75;
const  WHO_AM_I_VALUE: u8 = 0x71;
const PWR_MGMT_1: u8 = 0x6B;
const ACCEL_CONFIG: u8 = 0x1C;
const GYRO_CONFIG: u8 = 0x1B;
const ACCEL_XOUT_H: u8 = 0x3B;
const TEMP_OUT_H: u8 = 0x41;
const GYRO_XOUT_H: u8 = 0x43;
const SMPRT_DIV: u8 = 0x19;
const CONFIG: u8 = 0x1A;

//Main driver struct
pub struct Mpu9250<I2C> {
    i2c: I2C,
    address: u8,
    accel_scale: f32,   // Scaling factor for acceleration
    gyro_scale: f32,    // Scaling factor for gyroscope
}

pub enum AccelRange {
    Range2G,
    Range4G,
    Range8G,
    Range16G,
}

pub enum GyroRange {
    Range250Dps,
    Range500Dps,
    Range1000Dps,
    Range2000Dps,
}

pub enum DlpfConfig {
    Bandwidth260Hz,
    Bandwidth184Hz,
}

impl<I2C, E> Mpu9250<I2C>
where 
    I2C: I2c<Error = E>
{
    pub fn new(i2c : I2C, address: u8) -> Self {
        Mpu9250 {
            i2c: i2c, 
            address: address ,
            accel_scale: 0.0,     // Will be set during configuration 
            gyro_scale: 0.0 ,     // Will be set during configuration
        }
    }

    pub fn verify_identity(&mut self) -> Result<(), Error<E>> {
        // Buffer o hold the WHO_AM_I Register value
        let mut buffer = [0u8];

        // Write the register address we wanna read
        self.i2c.write(self.address, &[WHO_AM_I])?;

        // Read the value from that register
        self.i2c.read(self.address, &mut buffer)?;

        // Check if the value matches the expected Mpu9250 ID
        if buffer[0] != WHO_AM_I_VALUE {
            return Err(Error::NotDetected);
        }

        Ok(())
    }

    pub fn configure_power(&mut self) -> Result<(), Error<E>> {
        // Write to PWR_MGMT_1 register : 
        // - Device reset bit (Optional)
        // - Sleep mode disable
        // - Clock source selection (eg: internal oscillator)

        let config = 0x01;    // Wake up and use internal oscillator
        self.i2c.write(self.address, &[PWR_MGMT_1, config])?;

        Ok(())
    }

    pub fn setup_accelerometer(&mut self, range: AccelRange) -> Result<(), Error<E>> {
        let (config_value, scale) = match range {
            AccelRange::Range2G => (0x00, 2.0 / 32768.0),  //+-2g scale factor
            AccelRange::Range4G => (0x08, 4.0 / 32768.0),  //+-4g scale factor
            AccelRange::Range8G => (0x10, 2.0 / 32768.0),  //+-8g scale factor
            AccelRange::Range16G => (0x18, 16.0 / 32768.0),  //+-16g scale factor
        };

        // Write configuration to accelerometer config register
        self.i2c.write(self.address, &[ACCEL_CONFIG, config_value])?;

        // Store the scaling factor value for later data conversion
        self.accel_scale = scale;

        Ok(())
    }

    pub fn setup_gyroscope(&mut self, range: GyroRange) -> Result<(), Error<E>> {
        let   (config_value, scale) = match range {
            GyroRange::Range250Dps => (0x00, 250.0 / 32768.0),     // +-250 Dps
            GyroRange::Range500Dps => (0x08, 500.0 / 32768.0),     // +-500 Dps
            GyroRange::Range1000Dps => (0x10, 1000.0 / 32768.0),     // +-1000 Dps
            GyroRange::Range2000Dps => (0x18, 2000.0 / 32768.0),     // +-2000 Dps
        };

        //Write configuration to gyroscope config register
        self.i2c.write(self.address, &[GYRO_CONFIG, config_value])?;

        // Store the scaling factor for later data conversion
        self.gyro_scale = scale;

        Ok(())
    }

    pub fn initialize_sensor(&mut self, accel_range: AccelRange, gyro_range: GyroRange) -> Result<(), Error<E>> {
        self.verify_identity()?;
        self.configure_power()?;
        self.setup_accelerometer(accel_range)?;
        self.setup_gyroscope(gyro_range)?;

        Ok(())
    }
    
    pub fn read_accel_raw(&mut self) -> Result<[i16; 3], Error<E>> {
        let mut buffer = [0u8; 6];

        // Start reading from ACCEL_XOUT_H register 
        self.i2c.write(self.address, &[ACCEL_XOUT_H])?;

        // Read 6 bytes (X, Y and Z axes - 2 bytes each)
        self.i2c.read(self.address, &mut buffer)?;

        // Convert bytes to 16-bit signed integers
        let x = ((buffer[0] as i16) << 8) | buffer[1] as i16;
        let y = ((buffer[2] as i16) << 8) | buffer[3] as i16;
        let z = ((buffer[4] as i16) << 8) | buffer[5] as i16;

        Ok([x, y, z])
    }

    pub fn read_gyro_raw(&mut self) -> Result<[i16; 3], Error<E>> {
        let mut buffer = [0u8; 6];

        // Start reading from GYRO_XOUT_H register
        self.i2c.write(self.address, &[GYRO_XOUT_H])?;

        // Read 6 bytes (X, Y and Z axes - 2 bytes each)
        self.i2c.read(self.address, &mut buffer)?;

        // Convert bytes to 16-bit signed integers
        let x = ((buffer[0] as i16) << 8) | buffer[1] as i16;
        let y = ((buffer[2] as i16) << 8) | buffer[3] as i16;
        let z = ((buffer[4] as i16) << 8) | buffer[5] as i16;

        Ok([x, y, z])
    }

    pub fn read_temp_raw(&mut self) -> Result<i16, Error<E>> {
        let mut buffer = [0u8; 2];

        // Start reading from TEMP_OUT_H register
        self.i2c.write(self.address, &[TEMP_OUT_H])?;

        // Read 2 bytes
        self.i2c.read(self.address, &mut buffer)?;

        // Convert bytes to 16-bit signed integers
        let temp = ((buffer[0] as i16) << 8) | buffer[1] as i16;

        Ok(temp)
    }

    pub fn read_acceleration(&mut self) -> Result<[f32; 3], Error<E>> {
        let raw = self.read_accel_raw()?;

        // Apply scaling factor to convert to G-force
        let x = raw[0] as f32 * self.accel_scale;
        let y = raw[1] as f32 * self.accel_scale;
        let z = raw[2] as f32 * self.accel_scale;

        Ok([x, y, z])
    }

    pub fn read_angular_velocity(&mut self) -> Result<[f32; 3], Error<E>> {
        let raw = self.read_gyro_raw()?;

        // Apply scaling factor to convert to G-force
        let x = raw[0] as f32 * self.accel_scale;
        let y = raw[1] as f32 * self.accel_scale;
        let z = raw[2] as f32 * self.accel_scale;

        Ok([x, y, z])
    }

    pub fn read_temperature_celsius(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_temp_raw()?;

        // Apply scaling factor to convert to G-force
        let temperature = raw as f32 * self.accel_scale;

        Ok(temperature)
    }

    pub fn set_sample_rate(&mut self, divider: u8) -> Result<(), Error<E>> {
        // Write to sample rate divider register
        self.i2c.write(self.address, &[SMPRT_DIV, divider])?;
        Ok(())
    }

    pub fn set_dlpf_config(&mut self, config: DlpfConfig) -> Result<(), Error<E>> {
        let config_value = match config {
            DlpfConfig::Bandwidth260Hz => 0x00,
            DlpfConfig::Bandwidth184Hz => 0x01,
        };

        //Write to CONFIG register
        self.i2c.write(self.address, &[CONFIG, config_value])?;
        Ok(())
    }

    pub fn enter_sleep_mode(&mut self) -> Result<(), Error<E>> {
        // First read current PWR_MGMT_1 value
        let mut buffer = [0u8];
        self.i2c.write(self.address, &[PWR_MGMT_1])?;
        self.i2c.read(self.address, &mut buffer)?;

        // Set sleep bit (Bit 6)
        let new_config = buffer[0] | 0x40;

        // Write back with sleep bit set 
        self.i2c.write(self.address, &[PWR_MGMT_1, new_config])?;

        Ok(())
    }

   pub fn wake_up(&mut self) -> Result<(), Error<E>> {
       // First read the current PWR_MGMT_1 value 
       let mut buffer = [0u8];

       self.i2c.write(self.address, &[PWR_MGMT_1])?;
       self.i2c.read(self.address, &mut buffer)?;

       // Clear sleep bit (bit 6)
       let new_config = buffer[0] & 0x0F;

       // Write back with sleep bit cleared
       self.i2c.write(self.address, &[PWR_MGMT_1, new_config])?;

       Ok(())
   }
}


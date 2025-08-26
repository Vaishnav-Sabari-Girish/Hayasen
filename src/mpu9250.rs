#[cfg(feature = "mpu9250")]

use embedded_hal::i2c::I2c;
use crate::error::Error;

//Register addresses 
const WHO_AM_I: u8 = 0x75;
const PWR_MGMT_1: u8 = 0x6B;
const ACCEL_CONFIG: u8 = 0x1C;
const GYRO_CONFIG: u8 = 0x1B;
const ACCEL_XOUT_H: u8 = 0x3B;
const TEMP_OUT_H: u8 = 0x41;
const GYRO_XOUT_H: u8 = 0x43;

//Main driver struct
pub struct Mpu9250<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C, E> Mpu9250<I2C>
where 
    I2C: I2c<Error = E>
{
    pub fn new(i2c : I2C, address: u8) -> Self {
        Mpu9250 { i2c: i2c, address: address }
    }


}

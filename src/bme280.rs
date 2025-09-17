#[cfg(feature = "bme280")]
use embedded_hal::i2c::I2c;

#[cfg(feature = "bme280")]
use crate::error::Error;

#[cfg(feature = "bme280")]
mod registers {
    pub const ID: u8 = 0xD0;
    pub const RESET: u8 = 0xE0;
    pub const CTRL_HUM: u8 = 0xF2;
    pub const STATUS: u8 = 0xF3;
    pub const CTRL_MEAS: u8 = 0xF4;
    pub const CONFIG: u8 = 0xF5;

    //Data Registers
    pub const PRESS_MSB: u8 = 0xF7;
    pub const PRESS_LSB: u8 = 0xF8;
    pub const PRESS_XLSB: u8 = 0xF9;
    pub const TEMP_MSB: u8 = 0xFA;
    pub const TEMP_LSB: u8 = 0xFB;
    pub const TEMP_XLSB: u8 = 0xFC;
    pub const HUM_MSB: u8 = 0xFD;
    pub const HUM_LSB: u8 = 0xFE;

    // Calibration Registers
    pub const CALIB00: u8 = 0x88;
    pub const CALIB26: u8 = 0xE1;
}

#[cfg(feature = "bme280")]
use registers::*;

#[cfg(feature = "bme280")]
pub struct Bme280<I2C> {
    i2c: I2C,
    address: u8,
    calibration: CalibrationData,
    config: SensorConfig
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg(feature = "bme280")]
pub enum Oversampling {
    Skipped, 
    X1, 
    X2, 
    X4, 
    X8, 
    X16,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg(feature = "bme280")]
pub enum Mode {
    Sleep,
    Forced,
    Normal
}


#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg(feature = "bme280")]
pub enum StandbyTime {
    Ms0_5,
    Ms62_5,
    Ms125,
    Ms250,
    Ms500,
    Ms1000,
    Ms10,
    Ms20,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg(feature = "bme280")]
pub enum FilterCoefficient {
    Off,
    C2,
    C4,
    C8,
    C16
}

#[derive(Debug, Clone, Copy)]
#[cfg(feature = "bme280")]
struct CalibrationData {
    dig_t1 : u16,
    dig_t2 : i16,
    dig_t3 : i16,
    dig_p1 : u16,
    dig_p2 : i16,
    dig_p3 : i16,
    dig_p4 : i16,
    dig_p5 : i16,
    dig_p6 : i16,
    dig_p7 : i16,
    dig_p8 : i16,
    dig_p9 : i16,
    dig_h1 : u8,
    dig_h2 : i16,
    dig_h3 : u8,
    dig_h4 : i16,
    dig_h5 : i16,
    dig_h6 : i8,
}

#[derive(Debug, Clone, Copy)]
#[cfg(feature = "bme280")]
struct SensorConfig {
    temp_oversampling : Oversampling,
    press_oversampling : Oversampling,
    hum_oversampling : Oversampling,
    mode: Mode,
    standby_time : StandbyTime,
    filter : FilterCoefficient
}

#[cfg(feature = "bme280")]
impl<I2C, E> Bme280<I2C> 
where 
    I2C : I2c<Error = E>
{
    pub fn new(i2c : I2C, address: u8) -> Self {
        Bme280 {
            i2c,
            address,
            calibration : CalibrationData {
                dig_t1 : 0,
                dig_t2 : 0,
                dig_t3 : 0,
                dig_p1 : 0,
                dig_p2 : 0,
                dig_p3 : 0,
                dig_p4 : 0,
                dig_p5 : 0,
                dig_p6 : 0,
                dig_p7 : 0,
                dig_p8 : 0,
                dig_p9 : 0,
                dig_h1 : 0,
                dig_h2 : 0,
                dig_h3 : 0,
                dig_h4 : 0,
                dig_h5 : 0,
                dig_h6 : 0,
            },
            config : SensorConfig {
                temp_oversampling: Oversampling::X1,

            }
        }
    }
}

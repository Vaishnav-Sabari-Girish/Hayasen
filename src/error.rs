use core::fmt::{Debug, Formatter, Result};

pub enum Error<E> {
    I2c(E),
    InvalidData,
    NotDetected,
    ConfigError,
    SensorSpecific(&'static str),
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::I2c(error)
    }
}

//Debugging errors 
impl<E> Debug for Error<E> where E: Debug {
   fn fmt(&self, f: &mut Formatter) -> Result {
       match self {
           Error::I2c(e) => write!(f, "I2C Error: {:?}", e),
           Error::InvalidData => write!(f, "Invalid Data received from sensor"),
           Error::NotDetected => write!(f, "Sensor not detected at address"),
           Error::ConfigError => write!(f, "Invalid Configuration"),
           Error::SensorSpecific(msg) => write!(f, "Sensor Error : {}", msg),
       }
   } 
}

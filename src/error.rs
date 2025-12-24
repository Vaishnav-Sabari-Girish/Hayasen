use core::fmt::{Debug, Formatter, Result};

#[derive(Clone, PartialEq, Eq)]
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

impl<E> Debug for Error<E> 
where 
    E: Debug 
{

    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Error::I2c(e) => write!(f, "I2C Error: {:?}", e),
            Error::InvalidData => write!(f, "Invalid Data received from sensor"),
            Error::NotDetected => write!(f, "Sensor not detected at address"),
            Error::ConfigError => write!(f, "Invalid Configuration"),
            Error::SensorSpecific(msg) => write!(f, "Sensor Error: {}", msg),
        }
    }
}

// Additional convenience implementations
impl<E> Error<E> {

    pub fn is_i2c_error(&self) -> bool {
        matches!(self, Error::I2c(_))
    }
    

    pub fn is_config_error(&self) -> bool {
        matches!(self, Error::ConfigError | Error::SensorSpecific(_))
    }
    
    pub fn into_i2c_error(self) -> Option<E> {
        match self {
            Error::I2c(e) => Some(e),
            _ => None,
        }
    }
}

//Conditionally include features

#[cfg(feature = "mpu9250")]
pub mod mpu9250 {
    pub use crate::mpu9250::*;
}

# Hayasen 

Hayasen is an Embedded Rust library that hopes to make using sensors in Embedded Rust easier than ever. 

Currently it supports the following sensors 

1. MPU9250 (WIP)



## How to use 

Add `hayasen` library to your Cargo.toml as follows 

```toml
[dependencies]
hayasen = { version = "*", features = ["mpu9250"]}   # If you are going to use Mpu9250 sensor
```



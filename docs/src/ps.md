# Project Structure

This document outlines the high-level directory and file structure of the `hayasen` project, providing an
overview of where different components reside.

```
hayasen/
├── src/
│   ├── lib.rs          # Main library entry point, exposing the public API and core functionalities.
│   ├── error.rs        # Defines a unified set of error types for consistent error handling across the library.
│   ├── functions.rs    # Implements the function registry system, allowing dynamic management and execution of sensor-related operations.
│   └── mpu9250.rs      # Contains the implementation for interacting with the MPU9250 inertial measurement unit (IMU) sensor.
├── examples/           # Contains example code demonstrating how to use the `hayasen` library's features.
└── tests/              # Houses integration and unit tests to ensure the correctness and reliability of the library.
```

## Introduction

**Hayasen** is a lightweight, `no_std`-friendly Rust crate designed to provide a clean and modular interface
for a wide range of sensors. While initial support focuses on the **MPU9250** inertial measurement unit (IMU),
the architecture is built to easily extend to other sensors and communication protocols beyond I²C.

This crate is ideal for:

* **Embedded systems** running on microcontrollers (e.g., STM32, RP2040, ESP32, etc.).
* **Bare-metal Rust** applications requiring efficient, hardware-safe abstraction layers.
* Developers seeking **unified error handling**, modular sensor APIs, and minimal runtime overhead.

Key highlights:

* Extensible design for multiple sensor families.
* Unified error management through a generic `Error<E>` enum.
* Clear separation of concerns across modules for maintainability.
* `no_std` support for resource-constrained environments.

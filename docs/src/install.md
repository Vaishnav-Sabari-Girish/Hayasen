# Installation Guide for Hayasen

## 1. Prerequisites

* **Rust Toolchain**: Ensure that Rust (with Cargo) is installed. You can install it using:

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
* **Embedded Development Setup**:

  * Install the target for your microcontroller (e.g., ARM Cortex-M):

    ```bash
    rustup target add thumbv7em-none-eabihf
    ```
  * Ensure you have an appropriate linker and build tools installed.
* **Optional**: For cross-compilation, tools like `probe-rs` or `cargo-embed` can be used.

---

## 2. Adding Hayasen to Your Project

### Option A: Using `Cargo.toml`

Add the following line to your project's `Cargo.toml` under `[dependencies]`:

```toml
[dependencies]
hayasen = "*"
```

### Option B: Using `cargo add`

You can add Hayasen directly via Cargo command:

```bash
cargo add hayasen
```

This ensures that your project is always linked to the latest version of the crate.

Plans include adding each sensor as a feature to reduce overhead. It has already been implemented for the MPU9250. 

Therefore your full command is

```bash
cargo add hayasen --features mpu9250
```

---

## 3. Example Environment Setup

A minimal `Cargo.toml` for a project using Hayasen might look like:

```toml
[package]
name = "sensor_app"
version = "0.1.0"
edition = "2021"

[dependencies]
hayasen = "*"
embedded-hal = "0.2"
```

---

## 4. Verifying Installation

Run a test build to ensure everything is set up correctly:

```bash
cargo build --target thumbv7em-none-eabihf
```

If successful, you're ready to start integrating Hayasen with your embedded projects.

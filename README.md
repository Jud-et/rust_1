# Smart Watering System on Internet Computer Blockchain

This project is a backend canister written in Rust for a Smart Watering System. It monitors soil moisture levels and controls a water pump to automatically water the plants as needed. The canister is deployed on the Internet Computer (IC) blockchain.

## Prerequisites

- [DFINITY SDK](https://sdk.dfinity.org/)
- [Rust](https://www.rust-lang.org/)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [Node.js and npm](https://nodejs.org/)

## Project Setup

### 1. Install DFINITY SDK

```bash
sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
```

### 2. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable
rustup target add wasm32-unknown-unknown
```

### 3. Clone the Project

```bash
git clone https://github.com/Jud-et/rust_1.git
cd rust_1
```

### 5. Update Cargo.toml

Edit `Cargo.toml` to include the necessary dependencies:

```bash
[package]
name = "smart_watering_system"
version = "0.1.0"
edition = "2018"

[dependencies]
ic-cdk = "0.14.0"
ic-cdk-macros = "0.14.0"
candid = "0.10.9"
serde = "1.0"
serde_derive = "1.0"
serde_json = "1.0"

[lib]
crate-type = ["cdylib"]
```

### 6. Implement the Canister Logic

Edit `src/lib.rs`.

### 7. Generate the Candid Interface

Build the project and generate the Candid interface file:

```bash
cargo build --target wasm32-unknown-unknown --release
cargo test -- --nocapture
```

This will run the test that generates the `rust_1_backend.did` file in your project directory.

### 8. Update dfx.json

Edit `dfx.json` to include the Candid interface file:

```bash
{
  "canisters": {
    "smart_watering_system": {
      "main": "src/smart_watering_system/src/lib.rs",
      "type": "rust",
      "candid": "src/smart_watering_system/smart_watering_system.did"
    }
  },
  "networks": {
    "local": {
      "bind": "127.0.0.1:8000",
      "type": "ephemeral"
    }
  },
  "version": 1
}
```

### 9. Deploy the Canister

Deploy your canister to the local Internet Computer instance:

```bash
dfx deploy
```

## Interacting with the Canister

### Update Moisture Level

```bash
dfx canister call rust_1_backend update_moisture_level '(30.5)'
```

### Get Moisture Level

```bash
dfx canister call rust_1_backend get_moisture_level
```

### Get Moisture Log

```bash
dfx canister call rust_1_backend get_moisture_log
```

### Control Pump

```bash
dfx canister call rust_1_backend control_pump '(true)' # Turn on the pump
dfx canister call rust_1_backend control_pump '(false)' # Turn off the pump
```

### Get Pump State

```bash
dfx canister call rust_1_backend get_pump_state
```

# RISC-V Secure Boot Implementation

This is a secure boot implementation for RISC-V platforms using Rust. The implementation provides a secure boot process that verifies firmware integrity and authenticity before execution.

## Features

- Ed25519 signature verification
- SHA-256 hash verification
- Firmware integrity checking
- Secure boot configuration management
- Logging support for boot process

## Prerequisites

- Rust toolchain (2021 edition or later)
- RISC-V development environment
- Cargo package manager

## Project Structure

```
rustboot/
├── src/
│   └── main.rs          # Main secure boot implementation
├── Cargo.toml           # Project dependencies and configuration
├── firmware.bin         # Target firmware binary (not included)
├── boot.pub            # Public key for signature verification (not included)
└── firmware.sig        # Firmware signature file (not included)
```

## Building

```bash
cargo build --release
```

## Usage

1. Generate a keypair for firmware signing (using external tools)
2. Place your firmware binary as `firmware.bin`
3. Sign the firmware and save the signature as `firmware.sig`
4. Place the public key as `boot.pub`
5. Run the secure boot implementation

## Security Features

- Cryptographic signature verification using Ed25519
- Hash verification using SHA-256
- Early boot process validation
- Secure configuration loading

## Environment Variables

- `RUST_LOG`: Set logging level (e.g., `info`, `debug`, `warn`)

## License

This project is licensed under the MIT License - see the LICENSE file for details.

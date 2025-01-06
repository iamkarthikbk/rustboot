use log::{info, error};
use sha2::{Sha256, Digest};
use ed25519_dalek::{Verifier, VerifyingKey, Signature};
use std::fs;
use std::process;

const FIRMWARE_PATH: &str = "firmware.bin";
const PUBLIC_KEY_PATH: &str = "boot.pub";
const SIGNATURE_PATH: &str = "firmware.sig";

#[derive(Debug)]
struct SecureBootConfig {
    pub_key: VerifyingKey,
    firmware: Vec<u8>,
    signature: Signature,
}

fn load_secure_boot_config() -> Result<SecureBootConfig, Box<dyn std::error::Error>> {
    // Load public key
    let pub_key_bytes = fs::read(PUBLIC_KEY_PATH)?;
    if pub_key_bytes.len() != 32 {
        return Err("Invalid public key length".into());
    }
    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&pub_key_bytes);
    let pub_key = VerifyingKey::from_bytes(&key_bytes)?;

    // Load firmware
    let firmware = fs::read(FIRMWARE_PATH)?;

    // Load signature
    let sig_bytes = fs::read(SIGNATURE_PATH)?;
    if sig_bytes.len() != 64 {
        return Err("Invalid signature length".into());
    }
    let mut signature_bytes = [0u8; 64];
    signature_bytes.copy_from_slice(&sig_bytes);
    let signature = Signature::from_bytes(&signature_bytes);

    Ok(SecureBootConfig {
        pub_key,
        firmware,
        signature,
    })
}

fn verify_firmware(config: &SecureBootConfig) -> bool {
    match config.pub_key.verify(&config.firmware, &config.signature) {
        Ok(_) => {
            info!("Firmware signature verification successful");
            true
        }
        Err(e) => {
            error!("Firmware signature verification failed: {}", e);
            false
        }
    }
}

fn calculate_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

fn main() {
    // Initialize logging
    env_logger::init();
    info!("Starting RISC-V secure boot process...");

    // Load configuration
    let config = match load_secure_boot_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to load secure boot configuration: {}", e);
            process::exit(1);
        }
    };

    // Calculate and log firmware hash
    let firmware_hash = calculate_hash(&config.firmware);
    info!("Firmware hash: {}", firmware_hash);

    // Verify firmware signature
    if !verify_firmware(&config) {
        error!("Secure boot verification failed. Halting boot process.");
        process::exit(1);
    }

    info!("Secure boot verification successful!");
    // Here you would typically:
    // 1. Load the firmware into memory
    // 2. Verify memory regions
    // 3. Set up hardware security features
    // 4. Jump to firmware entry point
    
    info!("Transferring control to verified firmware...");
}

use std::path::Path;

fn main() {
    let key_path = Path::new("src/services/licensing/public_key.bin");

    if !key_path.exists() {
        // Write a 32-byte placeholder so include_bytes!() succeeds.
        // Licenses signed with the real private key will NOT verify against this key.
        std::fs::write(key_path, [0u8; 32])
            .expect("Failed to write placeholder public_key.bin");

        println!("cargo:warning=Using placeholder public_key.bin — license verification will always fail. Replace with the real public key for production.");
    }

    println!("cargo:rerun-if-changed=src/services/licensing/public_key.bin");
}

use std::fs;

pub fn init_flagd() {
    // Check if folder /etc/stable-os/flags exists
    if !fs::metadata("/etc/stable-os/flags").is_ok() {
        // Create folder /etc/stable-os/flags/old and all parent directories if they don't exist
        fs::create_dir_all("/etc/stable-os/flags/old").unwrap();
    }
}

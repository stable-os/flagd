mod flags;
mod init;
mod managers;
mod misc;

fn main() {
    println!("Starting flagd...");
    // Make sure flagd folder exists
    init::init_flagd();
    println!("Starting flagd... Done");

    println!("Performing flag checks...");
    let flag_changes = flags::check_flag_changes();
    println!("Performing flag checks... Done");

    println!("Running flag managers...");
    managers::run_flag_managers(flag_changes);
    println!("Running flag managers... Done");

    println!("Copying flags to /etc/stable-os/flags/old...");
    flags::copy_flags_to_old();
    println!("Copying flags to /etc/stable-os/flags/old... Done");
}

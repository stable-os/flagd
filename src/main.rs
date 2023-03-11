mod flags;
mod init;

fn main() {
    println!("Starting flagd...");
    // Make sure flagd folder exists
    init::init_flagd();

    println!("Starting flagd... Done");
    println!("Performing flag checks...");
    println!(
        "Changed flags (amount: {}): {:#?}",
        flags::check_flag_changes().len(),
        flags::check_flag_changes()
    );
    flags::copy_flags_to_old();
}

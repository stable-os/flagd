use std::fs;

#[derive(Debug, Clone)]
pub enum FlagChange {
    Added,
    Removed,
    Changed(String),
}

pub fn check_flag_changes() -> Vec<(String, FlagChange)> {
    let mut changed_flags: Vec<(String, FlagChange)> = Vec::new();
    let mut old_flags: Vec<String> = Vec::new();
    let mut new_flags: Vec<String> = Vec::new();

    // Get all files in /etc/stable-os/flags/old
    for entry in fs::read_dir("/etc/stable-os/flags/old").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        old_flags.push(file_name);
    }

    // Get all files in /etc/stable-os/flags
    for entry in fs::read_dir("/etc/stable-os/flags").unwrap() {
        let entry = entry.unwrap();

        // Check if the file is a file
        if !entry.file_type().unwrap().is_file() {
            continue;
        }

        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        new_flags.push(file_name);
    }

    // Check if any flags have been added
    for flag in new_flags.clone() {
        if !old_flags.contains(&flag) {
            changed_flags.push((flag, FlagChange::Added));
        }
    }

    // Check if any flags have been removed
    for flag in old_flags.clone() {
        if !new_flags.contains(&flag) {
            changed_flags.push((flag, FlagChange::Removed));
        }
    }

    // Check if the contents of any flags have changed
    for flag in new_flags {
        // Check if the flag exists in /etc/stable-os/flags/old
        if !old_flags.contains(&flag) {
            continue;
        }

        let old_flag = fs::read_to_string(format!("/etc/stable-os/flags/old/{}", flag)).unwrap();
        let new_flag = fs::read_to_string(format!("/etc/stable-os/flags/{}", flag)).unwrap();
        if old_flag != new_flag {
            changed_flags.push((flag, FlagChange::Changed(new_flag)));
        }
    }

    // Return a list of changed flags
    changed_flags
}

pub fn copy_flags_to_old() {
    // Delete all files in /etc/stable-os/flags/old
    for entry in fs::read_dir("/etc/stable-os/flags/old").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        fs::remove_file(format!("/etc/stable-os/flags/old/{}", file_name)).unwrap();
    }

    // Get all files in /etc/stable-os/flags
    for entry in fs::read_dir("/etc/stable-os/flags").unwrap() {
        let entry = entry.unwrap();

        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

        // Check if the item is a file
        if !entry.file_type().unwrap().is_file() {
            continue;
        }

        // Copy the file to /etc/stable-os/flags/old
        fs::copy(
            format!("/etc/stable-os/flags/{}", file_name),
            format!("/etc/stable-os/flags/old/{}", file_name),
        )
        .unwrap();
    }
}

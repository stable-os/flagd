use crate::{flags::FlagChange, managers::FlagManager, misc::execute_command};

pub const refind_enabled: FlagManager =
    FlagManager("boot.bootmanager.refind_enabled", &refind_enabled_callback);
fn refind_enabled_callback(flag_name: String, flag_change: FlagChange) {
    match flag_change {
        FlagChange::Added => execute_command("refind-install"),
        FlagChange::Removed => {
            execute_command("rm -r /boot/efi/EFI/refind");
        }
        _ => {}
    }
}

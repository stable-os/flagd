use crate::flags::FlagChange;

mod boot;

pub struct FlagManager<'a>(&'a str, &'a dyn Fn(String, FlagChange));

fn managers() -> Vec<FlagManager<'static>> {
    vec![boot::bootmanager::refind_enabled]
}

pub fn run_flag_managers(changed_flags: Vec<(String, FlagChange)>) {
    for flag_manager in managers() {
        for changed_flag in &changed_flags {
            if changed_flag.0 == flag_manager.0 {
                (flag_manager.1)(changed_flag.0.clone(), changed_flag.1.clone());
            }
        }
    }
}

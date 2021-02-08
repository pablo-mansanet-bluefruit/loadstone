#![cfg_attr(test, allow(unused_attributes))]
#![cfg_attr(all(not(test), target_arch = "arm"), no_std)]
#![cfg_attr(target_arch = "arm", no_main)]

#[allow(unused_imports)]
use cortex_m_rt::{entry, exception};

pub const GREETING: &str =
    "--=Loadstone demo app CLI + Boot Manager=--\ntype `help` for a list of commands.";

#[cfg(target_arch = "arm")]
#[entry]
fn main() -> ! {
    use loadstone_lib::devices::boot_manager;
    let app = boot_manager::BootManager::new();
    app.run(GREETING);
}

#[cfg(not(target_arch = "arm"))]
fn main() {}

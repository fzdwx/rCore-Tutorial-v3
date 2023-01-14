//! The panic handler

use crate::sbi::shutdown;
use core::panic::PanicInfo;
use log::info;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        info!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        info!("[kernel] Panicked: {}", info.message().unwrap());
    }
    shutdown()
}

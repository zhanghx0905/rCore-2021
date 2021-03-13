use core::panic::PanicInfo;
use crate::sbi::shutdown;
use log::info;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        info!("[kernel] Panicked at {}:{} {}", location.file(), location.line(), info.message().unwrap());
    } else {
        info!("[kernel] Panicked: {}", info.message().unwrap());
    }
    shutdown()
}

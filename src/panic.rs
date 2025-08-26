use core::panic::PanicInfo;

pub use crate::info;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    let mut count = 5;
    loop {
        count -= 1;
        info!("x.");
        if count == 0 {
            info!("..:");
            syscall::exit(23);
        }
    }
}

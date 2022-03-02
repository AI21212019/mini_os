#![no_std] // don't link the Rust standard library

use ::core::panic::PanicInfo;
fn main() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

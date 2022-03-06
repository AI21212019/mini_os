#![no_std] // don't link the Rust standard library
#![no_main] //disable all Rust-level entry points

use ::core::panic::PanicInfo;

mod vga_buffer;
//main fn is removed due to no use of runtime.

#[no_mangle] //don't mangle the name of this function
pub extern "C" fn _start() {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    println!("Hello World{}", "!");
    // panic!("Some panic message"); //example of panic to be display on the virtual machine

    loop {}
}
//this fn is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

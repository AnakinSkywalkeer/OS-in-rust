// in tests/basic_boot.rs

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "tes"]

use core::panic::PanicInfo;
use blog_os::println;
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    tes();

    loop {}
}



#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
#[test_case]
fn idk(){
    println!("Hello world");
}
#![no_std]
#![no_main]
mod vga_buffer;
use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info:&PanicInfo)-> !{
    loop{}
}

static HELLO:&[u8]=b"Hello world Gary this side";
#[no_mangle]
pub extern "C" fn _start()-> !{
    vga_buffer::print_something();
    loop{}
}



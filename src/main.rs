#![no_std]
#![no_main]
mod vga_buffer;
use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info:&PanicInfo)-> !{
    loop{}
}


#[no_mangle]
pub extern "C" fn _start()-> !{
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again Gary this side").unwrap();
    write!(vga_buffer::WRITER.lock(),"Some numbers {} {}",0.78,90).unwrap();
    loop{}
}




#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "tes"]
#[allow(unconditional_panic)]

use core::panic::PanicInfo;

use blog_os::vga_buffer::WRITER;
use blog_os::vga_buffer::Writer;
pub mod vga_buffer;
use core::time::Duration;
use core::arch::x86_64::_rdtsc;
use core::arch::asm;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // println!("Hello world");
    blog_os::init(); // new

    


    // for i in 1..20{
    //     unsafe {
    //         let mut smth: u64 = _rdtsc();
    //         loop {
    //           let new_smth: u64 = _rdtsc();
    //           if  new_smth > (smth + 900000000) {
    //             WRITER.lock().putApple(i);
    //             break;
    //           }
    //           let smth = new_smth;
    //         }
    //       }
    // }
    

    #[cfg(test)]
    tes();

    blog_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
}
fn idk(){
  unsafe{
    asm!(
      "mov ah, 7",
      "mov bl, 0",
      "div bl",
    );
 }
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
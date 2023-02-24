#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "tes"]

mod vga_buffer;
use core::panic::PanicInfo;
use core::fmt::Write;



#[panic_handler]
fn panic(info:&PanicInfo)-> !{
    println!("{}",info);
    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    println!("Hello world");

    #[cfg(test)]
    tes();
    
    println!("It did not crash!!!");
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}
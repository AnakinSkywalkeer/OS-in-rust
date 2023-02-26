#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main="tes"]
use core::panic::PanicInfo;
use blog_os::{QemuExitCode,exit_qemu,serial_println,serial_print};


#[panic_handler]
fn panic(_info:&PanicInfo)->!{
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop{}
}

#[no_mangle]
pub extern "C" fn _start()->!{
    should_panic();
    serial_println!("[test did not pacnic]");
    exit_qemu(QemuExitCode::Failed);
    loop{}
}

fn should_panic(){
    serial_print!("Shoud_panic::should_fail...\t");
    assert_eq!(0,1);
}
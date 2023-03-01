// in src/lib.rs
#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "tes"]
#![feature(abi_x86_interrupt)]
use core::panic::PanicInfo;
pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    tes();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
// in src/lib.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
//--------------------------------------
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}
#[derive(Debug, Clone, Copy)]
pub struct Snake {
    direction: Direction,
    row: u32,
    col: u32,
}
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;
use vga_buffer::WRITER;
use vga_buffer::Writer;
use pc_keyboard::KeyCode;
impl Snake {
    pub fn make_snake() -> Snake {
        let snake: Snake = Snake {
            direction: Direction::RIGHT,
            row: 1,
            col: 0,
        };
        snake
    }
    
    pub fn dir(&mut self, a: DecodedKey){
        match a {
            DecodedKey::RawKey(key) if key == KeyCode::ArrowLeft => {
                use core::arch::x86_64::_rdtsc;
                unsafe {
                    let mut smth: u64 = _rdtsc();
                    loop {
                      let new_smth: u64 = _rdtsc();
                      if  new_smth > (smth + 900000000) {
                        WRITER.lock().putApple();
                        break;
                      }
                      let smth = new_smth;
                    }
                  }
            },
            DecodedKey::RawKey(key) if key == KeyCode::ArrowRight =>{
                use core::arch::x86_64::_rdtsc;
                unsafe {
                    let mut smth: u64 = _rdtsc();
                   
                    loop {
                      let new_smth: u64 = _rdtsc();
                      if  new_smth > (smth + 900000000) {
                        WRITER.lock().opp();
                        break;
                      }
                      let smth = new_smth;
                    }
                  }
            },
            DecodedKey::Unicode(character) => self.direction=self.direction,
           _=>self.direction=self.direction,
        }
    }
    pub fn get(&mut self)->Direction{
        self.direction
    }
}
use lazy_static::lazy_static;
lazy_static!{
    pub static ref SNAKE:Mutex<Snake>=Mutex::new(Snake::make_snake());
}
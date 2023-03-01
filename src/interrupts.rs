#![feature(abi_x86_interrupt)]
use pc_keyboard::KeyCode;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::Snake;
use crate::println;
use lazy_static::lazy_static;
use core::arch::asm;
use pic8259::ChainedPics;
use spin;
use crate::print;
use crate::vga_buffer;
use crate::gdt;

use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;
use vga_buffer::WRITER;
use vga_buffer::Writer;



lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt.divide_error.set_handler_fn(divide_zero);
        
        unsafe{
            idt.double_fault.set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame:InterruptStackFrame){
      println!("EXCEPTION: BREAKPOINT\n{:#?}",stack_frame);
}

extern "x86-interrupt" fn divide_zero(stack_frame:InterruptStackFrame){
    
    
    println!("ARE U DUMB U ARE TRYING TO DIVIDE SMTH BY ZERO GO READ UR TEXT BOOK\n{:#?}",stack_frame);
    panic!("ZERO DIVISION ERROR");
}
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}

//hardware interrupts
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub const PIC_1_OFFSET:u8=32;
pub const  PIC_2_OFFSET:u8=PIC_1_OFFSET+8;

pub static PICS:spin::Mutex<ChainedPics>=spin::Mutex::new(unsafe{ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)});
extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    // print!(".");
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}
extern  "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame:InterruptStackFrame
){
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;
    use vga_buffer::WRITER;
    use vga_buffer::Writer;   
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1,
                HandleControl::Ignore)
            );
    }
    let mut keyboard=KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) if character == (8) as char =>WRITER.lock().backspace(),
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key)if key==KeyCode::ArrowLeft || key==KeyCode::ArrowRight=> {crate::SNAKE.lock().dir(DecodedKey::RawKey(key));},//arrows //print!("{:?}", key)//crate::SNAKE.lock().dir(DecodedKey::RawKey(key)) 
                _=>print!("hello"),//crate::SNAKE.lock().dir(key)
            }
            //print!("{:?}",crate::SNAKE.lock().get())
        }
    }
    unsafe{
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}




//-------------------



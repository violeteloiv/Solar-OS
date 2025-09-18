#![allow(bad_asm_style, clippy::missing_safety_doc)]
#![no_std]
#![no_main]

#[macro_use]
mod vga;
mod libc;
mod multiboot;

use multiboot::MultibootInfo;
use vga::TerminalWriter;

use core::{arch::global_asm, panic::PanicInfo};

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub unsafe extern "C" fn kernel_main(_multiboot_magic: u32, info: *const MultibootInfo) -> i32 {
    TerminalWriter::init();
    unsafe {
        multiboot::print_mmap_sections(info);
    }
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
#![allow(bad_asm_style, clippy::missing_safety_doc)]
#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

#[macro_use]
mod io;
#[cfg(test)]
#[macro_use]
mod testing;
mod allocator;
mod libc;
mod multiboot;

use multiboot::MultibootInfo;
use io::vga::TerminalWriter;

use core::{arch::global_asm, panic::PanicInfo};
use alloc::vec;

global_asm!(include_str!("boot.s"));

extern "C" {
    #[no_mangle]
    static KERNEL_START: u32;
    #[no_mangle]
    static KERNEL_END: u32;
}

fn test_runner(test_fns: &[&dyn Fn()]) {
    for test_fn in test_fns {
        test_fn();
    }
}

#[no_mangle]
pub unsafe extern "C" fn kernel_main(_multiboot_magic: u32, info: *const MultibootInfo) -> i32 {
    TerminalWriter::init();
    io::serial::Serial::init().expect("Failed To Initialize Serial Output");
    allocator::ALLOC.init(&*info);

    #[cfg(test)]
    {
        test_main();
        io::exit(0);
    }

    println!("Kernel Range: {:?} -> {:?}", &KERNEL_START as *const u32, &KERNEL_END as *const u32);
    println!("");

    let v = vec![0, 1, 2, 3];
    println!("{:?}", v);

    let x = 3;
    let y = "test";
    println!("Hello! Here's an int: {x}, here's a float: {}", 1.1 + 2.2);
    println!("and here's a string: '{y}'");
    println!("");

    unsafe {
        multiboot::print_mmap_sections(info);
    }
    
    io::exit(0);
    0
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    if let Some(location) = panic_info.location() {
        print!(
            "Solar Kernel Panic at {}:{} | ",
            location.file(),
            location.line()
        );
    } else {
        print!("Solar Kernel Panic At Unknown Location | ");
    }
    println!("{}", panic_info.message());
    unsafe { io::exit(1); }
    loop {}
}
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

use core::{arch::global_asm, panic::PanicInfo};
use alloc::vec;

global_asm!(include_str!("boot.s"));

extern "C" {
    #[no_mangle]
    static KERNEL_START: u32;
    #[no_mangle]
    static KERNEL_END: u32;
}

#[no_mangle]
pub unsafe extern "C" fn kernel_main(_multiboot_magic: u32, info: *const MultibootInfo) -> i32 {
    allocator::init(&*info);
    let mut port_manager = io::port_manager::PortManager::new();
    io::init_stdio(&mut port_manager);
    io::init_late(&mut port_manager);

    #[cfg(test)]
    {
        test_main();
        io::exit(0);
    }

    println!("A vector: {:?}", vec![1, 2, 3, 4, 5]);
    let a_map: hashbrown::HashMap<&'static str, i32> = [("test", 1), ("test2", 2)].into_iter().collect();
    println!("A map: {:?}", a_map);
    println!("");
    
    println!("Exit/Halting");
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
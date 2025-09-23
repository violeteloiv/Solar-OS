#![allow(bad_asm_style, clippy::missing_safety_doc)]
#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
mod vga;
mod allocator;
mod libc;
mod multiboot;

use multiboot::MultibootInfo;
use vga::TerminalWriter;

use core::{arch::global_asm, panic::PanicInfo, sync::atomic::Ordering};
use alloc::{vec, vec::Vec};

#[global_allocator]
static ALLOC: allocator::Allocator = allocator::Allocator::new();

global_asm!(include_str!("boot.s"));

extern "C" {
    #[no_mangle]
    static KERNEL_START: u32;
    #[no_mangle]
    static KERNEL_END: u32;
}

#[no_mangle]
pub unsafe extern "C" fn kernel_main(_multiboot_magic: u32, info: *const MultibootInfo) -> i32 {
    TerminalWriter::init();
    
    println!("Kernel Range: {:?} -> {:?}", &KERNEL_START as *const u32, &KERNEL_END as *const u32);
    println!("");

    ALLOC.init(&*info);

    let initial_state = ALLOC.first_free.load(Ordering::Relaxed);

    {
        let mut v = Vec::new();
        const NUM_ALLOCS: usize = 5;
        for i in 0..NUM_ALLOCS {
            let mut v2 = Vec::new();
            for j in 0..i {
                v2.push(j);
            }
            v.push(v2);
        }

        for i in (0..(NUM_ALLOCS - 1)).filter(|x| (x % 2) == 0).rev() {
            let len = v.len() - 1;
            v.swap(len, i);
            v.pop();
        }

        {
            let mut v = Vec::new();
            for i in 0..NUM_ALLOCS {
                let mut v2 = Vec::new();
                for j in 0..i {
                    v2.push(j);
                }
                v.push(v2);
            } 
        }

        println!("Pre Dealloc");
        allocator::print_all_free_segments(ALLOC.first_free.load(Ordering::Relaxed));

        for elem in v {
            for (i, item) in elem.iter().enumerate() {
                assert_eq!(i, *item);
            }
        }
    }

    println!("Post Dealloc");
    allocator::print_all_free_segments(ALLOC.first_free.load(Ordering::Relaxed));
    assert_eq!(ALLOC.first_free.load(Ordering::Relaxed), initial_state);

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
    loop {}
}
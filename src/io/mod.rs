#[macro_use]
pub mod vga;
pub mod serial;
use core::arch::asm;

macro_rules! print {
    ($($arg:tt)*) => {
        #[allow(unused_unsafe)]
        unsafe {
            use core::fmt::Write as FmtWrite;
            // write_fmt needs writer as &mut, but we only access it as *const. Cast to fulfil the
            // API requirements
            let writer = &mut *$crate::io::vga::TERMINAL_WRITER.inner.get();
            write!(&mut *(writer), $($arg)*).expect("Failed to print to vga");
            let serial = &mut *$crate::io::serial::SERIAL.inner.get();
            write!(&mut *(serial), $($arg)*).expect("Failed to print to serial");
        }
    }
}

macro_rules! println {
    ($($arg:tt)*) => {
        print!($($arg)*);
        print!("\n");
    }
}

unsafe fn inb(addr: u16) -> u8 {
    let mut ret;
    asm!(r#"
        .att_syntax
        in %dx, %al
        "#, 
        in("dx") addr,
        out("al") ret);
    ret
}

unsafe fn outb(addr: u16, val: u8) {
    asm!(r#"
        .att_syntax
        out %al, %dx
        "#, 
        in("dx") addr,
        in("al") val);
}

pub unsafe fn exit(code: u8) {
    const ISA_DEBUG_EXIT_PORT: u16 = 0xf4;
    outb(ISA_DEBUG_EXIT_PORT, code);
}
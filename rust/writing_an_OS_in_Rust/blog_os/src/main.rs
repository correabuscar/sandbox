//src: https://os.phil-opp.com/freestanding-rust-binary/
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
use core::arch::asm;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xc;
        }
    }

    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    //x86_64::instructions::interrupts::disable();
    loop {
        //x86_64::instructions::hlt();
        //x86_64::instructions::interrupts::enable_and_hlt();
        unsafe {
            asm!("hlt", options(nomem, nostack, preserves_flags));
        }
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    //x86_64::instructions::interrupts::disable();
    loop {
        //x86_64::instructions::hlt();
        //x86_64::instructions::interrupts::enable_and_hlt();
        unsafe {
            asm!("hlt", options(nomem, nostack, preserves_flags));
        }
    }
}




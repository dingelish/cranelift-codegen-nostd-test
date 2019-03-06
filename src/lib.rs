#![no_std]
#![feature(lang_items)]
use core::panic::PanicInfo;

use cranelift_codegen;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unimplemented!()
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

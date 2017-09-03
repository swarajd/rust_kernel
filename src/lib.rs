
// used to allow the specified feature-gated attributes in this crate. (only in nightly)
#![feature(lang_items)]

// we don't want std because std relies on OS stuff
#![no_std]

// add the libc functions
extern crate rlibc;

// without ! applies to following item
#[no_mangle]
pub extern fn rust_main() {
    // ATTENTION: we have a very small stack and no guard page

    let hello = b"Hello World!";
    let color_byte = 0x1f; //white foreground, blue background

    let mut hello_colored = [color_byte; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte;
    }

    let buffer_ptr = (0xb8000 + 1988) as *mut _;
    unsafe { *buffer_ptr = hello_colored };

    loop {}
}

// used for Rust's unwinding on panic!
#[lang = "eh_personality"] extern fn eh_personality() {

}

// the entry point for panic
// we make sure it doesn't return using "!"
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {
    loop {

    }
}
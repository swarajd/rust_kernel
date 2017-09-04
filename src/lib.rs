
// used to allow the specified feature-gated attributes in this crate. (only in nightly)
#![feature(lang_items)]
#![feature(unique)]
#![feature(const_fn)]

// we don't want std because std relies on OS stuff
#![no_std]

// add the libc functions
#[allow(unused_extern_crates)]
extern crate rlibc;


// add the volatile to make sure compilers don't optimize out writes
extern crate volatile;

// add the spin lock for the writer
extern crate spin;

#[macro_use]
mod vga_buffer;

// without ! applies to following item
#[no_mangle]
#[allow(unused_must_use)]
pub extern fn rust_main() {
    // ATTENTION: we have a very small stack and no guard page

    vga_buffer::clear_screen();
    println!("Hello World{}", "!");
    println!("{}", { println!("inner"); "outer" });
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
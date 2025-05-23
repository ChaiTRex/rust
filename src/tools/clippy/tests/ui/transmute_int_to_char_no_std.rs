#![no_std]
#![feature(lang_items)]
#![warn(clippy::transmute_int_to_char)]
#![allow(clippy::missing_transmute_annotations, unnecessary_transmutes)]

use core::panic::PanicInfo;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

fn int_to_char() {
    let _: char = unsafe { core::mem::transmute(0_u32) };
    //~^ transmute_int_to_char

    let _: char = unsafe { core::mem::transmute(0_i32) };
    //~^ transmute_int_to_char

    // These shouldn't warn
    const _: char = unsafe { core::mem::transmute(0_u32) };
    const _: char = unsafe { core::mem::transmute(0_i32) };
}

fn main() {}

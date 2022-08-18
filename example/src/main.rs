#![no_std]
#![no_main]

extern crate gba_minrt;

#[no_mangle]
pub fn main() {
    unsafe {
        (0x0400_0000 as *mut u16).write_volatile(0x0403);

        (0x0600_0000 as *mut u16).offset(120 + 80 * 240).write(0x001F);
        (0x0600_0000 as *mut u16).offset(136 + 80 * 240).write(0x03E0);
        (0x0600_0000 as *mut u16).offset(120 + 96 * 240).write(0x7C00);
    }
}

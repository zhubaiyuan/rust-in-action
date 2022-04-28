use core::arch::asm;

fn main() {
    unsafe {
        asm!("int 42");
    }
}
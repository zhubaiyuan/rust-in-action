fn main() {
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;
    println!("base 10: {} {} {}", three, thirty, three_hundred);
    // base 10: 3 30 300
    println!("base 2:  {:b} {:b} {:b}", three, thirty, three_hundred);
    // base 2:  11 11110 100101100
    println!("base 8:  {:o} {:o} {:o}", three, thirty, three_hundred);
    // base 8:  3 36 454
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
    // base 16: 3 1e 12c
}

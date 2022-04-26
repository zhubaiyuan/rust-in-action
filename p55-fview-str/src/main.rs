use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;

fn main() -> std::io::Result<()> {
    let mut input: &'static [u8] = br#"
    fn main() {
        println!("Hello, world!");
    }"#;
    let mut buffer: Vec<u8> = vec![];
    input.read_to_end(&mut buffer)?;
    let mut position_in_input = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        print!("[0x{:08x}] ", position_in_input);
        for byte in line {
            print!("{:02x} ", byte);
        }
        println!();
        position_in_input += BYTES_PER_LINE;
    }
    Ok(())
}

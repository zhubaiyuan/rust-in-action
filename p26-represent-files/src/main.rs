#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }
}

#[allow(dead_code)]
fn open(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = f.data.clone();
    let read_length = tmp.len();
    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    read_length
}

fn main() {
    let f3 = File::new("f3.txt");
    let f3_name = &f3.name;
    let f3_length = f3.data.len();
    println!("{:?}", f3);
    println!("{} is {} bytes long", f3_name, f3_length);
}

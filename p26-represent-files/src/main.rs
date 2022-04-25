//! Simulating files one step at a time.
#![allow(dead_code)]

use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

/// Represents a "file",
/// which probably lives on a file system.
#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    /// New files are assumed to be empty, but a name is required.
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }

    /// Returns the file's length in bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the file's name.
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let f1 = File::new("f1.txt");
    let f1_name = f1.name();
    let f1_length = f1.len();
    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}

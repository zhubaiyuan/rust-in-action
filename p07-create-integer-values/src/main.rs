use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    let a = 10;
    let b = Box::new(20);
    let c = Rc::new(Box::new(30));
    let d = Arc::new(Mutex::new(40));
    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
    // a: 10, b: 20, c: 30, d: Mutex { data: 40, poisoned: false, .. }
}

use std::{thread, time};

#[allow(unused_must_use)]
fn main() {
    let pause = time::Duration::from_millis(20);
    let handle1 = thread::spawn(move || {
        thread::sleep(pause);
    });
    let handle2 = thread::spawn(move || {
        thread::sleep(pause);
    });
    handle1.join();
    handle2.join();
}

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
#[repr(C)]
struct InputEvent {
    tv_sec: isize, // timeval struct
    tv_usec: isize, // timeval struct
    type_: u16,
    code: u16,
    value: i32
}

fn main() {
    let mut input = File::open("/dev/input/event7").expect("Input device 7 does not exist");
    let mut buffer : [u8; 24] = [0; 24];

    loop {
        while let Ok(()) = input.read_exact(&mut buffer) {
            let event: InputEvent = unsafe { std::mem::transmute(buffer) };
            println!("{:?}", event);
        }
    }
}

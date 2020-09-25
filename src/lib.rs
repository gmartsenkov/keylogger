#[derive(Debug)]
#[repr(C)]
pub struct InputEvent {
    tv_sec: isize, // timeval struct
    tv_usec: isize, // timeval struct
    type_: u16,
    code: u16,
    value: i32
}

impl InputEvent {
    pub fn key_pressed(&self) -> Option<u16> {
        if self.type_ == 1 && self.value == 1 {
            return Some(self.code);
        }
        None
    }
}

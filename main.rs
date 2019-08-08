#![no_std]

#[derive(Copy, Clone)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Pink       = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    LightPink  = 13,
    Yellow     = 14,
    White      = 15,
}

struct IntRange {
    cur: u32,
    max: u32
}

impl IntRange {
    fn next(&mut self) -> Option<u32> {
        if self.cur < self.max {
            self.cur += 1;
            Some(self.cur - 1)
        } else {
            None
        }
    }
}

fn range(lo: u32, hi: u32) -> IntRange {
    IntRange { cur: lo, max: hi }
}

const VGA_WIDTH: u16 = 20;
const VGA_ADDRESS: u32 = 0xb8000;

fn clear_screen(background: Color) {
    let mut r = range(0, 80 * 25);
    loop{
        match r.next() {
            Some(x) => {
                unsafe {
                    *((VGA_ADDRESS + x * 2) as *mut u16) = (background as u16) << 12;
                }
            },
            None =>{break}
        }
    }
}

#[no_mangle]
//  #[no_split_stack]
pub fn main() {
    clear_screen(Color::LightRed);
}

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

struct TextCursor {
    x: u16,
    y: u16,
    fg_color: Color,
    bg_color: Color
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

const VGA_WIDTH: u16 = 80; // In Characters
const VGA_HEIGHT: u16 = 25;
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
    //clear_screen(Color::LightGreen);

    let mut cursor = TextCursor{
        x:0,
        y:0,
        fg_color:Color::White,
        bg_color:Color::DarkGray
    };

    // Say Hello
    cursor = println(cursor, "Hello");
    // Say World
    cursor = println(cursor, "World");
    cursor = println(cursor, "\nThis is\na test.");
}

fn println(mut cursor:TextCursor, text:&str) -> TextCursor {
    let bytes : &[u8]= text.as_bytes();

    for b in bytes {
        if b == &("\n".as_bytes()[0]) {
            cursor.x = 0;
            cursor.y = cursor.y + 1;
        } else {
            putchar(&cursor, b);
            cursor.x = cursor.x + 1;
        }
    }

    cursor.x = 0;
    cursor.y = cursor.y + 1;

    cursor
}

// Copying code from Julia Evans: https://jvns.ca/blog/2014/03/12/the-rust-os-story/
fn putchar(cursor:&TextCursor, c:&u8) {
    let idx : u32 = (cursor.y * VGA_WIDTH * 2 + cursor.x * 2).into();
    let fg_color = cursor.fg_color;
    let bg_color = cursor.bg_color;

    unsafe {
        *((VGA_ADDRESS + idx) as *mut u16) = make_vgaentry(*c, fg_color, bg_color);
    }
}

fn make_vgaentry(c: u8, fg: Color, bg: Color) -> u16 {
    let color = fg as u16 | ((bg as u16) << 4);
    return c as u16 | (color << 8);
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Character {
    char: u8,
    color: CharacterColor,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u8)]
enum CharacterColor {
    Black = 0,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightMagenta,
    Yellow,
    White,
}

pub fn write() {
    // println("Omar EmadoooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooEmadoooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooEmadooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo");
    // println("Omar Emad");
    // println("mohamed");
    println("khal");
}


pub fn println(text: &str) {
    let mut writer = WRITER.lock();

    for byte in text.bytes() {
        writer.write_byte(byte);
    }
}

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

struct Buffer {
    chars: [[core::ptr::NonNull<Character>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

impl Buffer {
    pub fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.chars[row][col];
                self.chars[row - 1][col] = character;
            }
        }
    }
}

pub struct Writer {
    column_position: usize,
    buffer: *mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        let buffer = self.buffer;
        unsafe {
            (*buffer).chars[0][0].write_volatile(Character {
                char: b'O',
                color: CharacterColor::White,
            });
            self.column_position += 1;
        }
    }
}

// Promise that Writer can be sent to other threads
unsafe impl Send for Writer {}

// Promise that Writer can be shared between threads (needed for static Mutex)
unsafe impl Sync for Writer {}

use spin::Mutex;

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    buffer: 0xb8000 as *mut Buffer,
});

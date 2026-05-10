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
    println(
        "Omar EmadoooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooEmadoooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooEmadooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo",
    );
    println("Omar Emad");
    println("mohamed");
    println("khal");
}

pub fn println(text: &str) {
    let mut writer = WRITER.lock();

    writer.write_byte(b'\n');
    for byte in text.bytes() {
        writer.write_byte(byte);
    }
}

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[repr(transparent)]
struct Buffer {
    chars: [[Character; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Buffer {
    pub fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                // Inside Buffer::new_line
                let character =
                    unsafe { core::ptr::read_volatile(&self.chars[row][col] as *const Character) };
                unsafe {
                    core::ptr::write_volatile(
                        &mut self.chars[row - 1][col] as *mut Character,
                        character,
                    );
                }
            }
        }
        for col in 0..BUFFER_WIDTH {
            let char = Character {
                char: b' ',
                color: CharacterColor::White,
            };
            unsafe {
                core::ptr::write_volatile(
                    &mut self.chars[BUFFER_HEIGHT - 1][col] as *mut Character,
                    char,
                );
            };
        }
    }
}

unsafe impl Send for Writer {}
unsafe impl Sync for Writer {}

pub struct Writer {
    column_position: usize,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.column_position = 0;
                self.buffer.new_line();
            }
            b'\t' => {
                self.column_position += 4;
                if self.column_position >= BUFFER_WIDTH {
                    self.column_position = 0;
                    self.buffer.new_line();
                }
            }
            _ => {
                (*self.buffer).chars[BUFFER_HEIGHT - 1][self.column_position] = Character {
                    char: byte,
                    color: CharacterColor::White,
                };
                if self.column_position == BUFFER_WIDTH - 1 {
                    self.buffer.new_line();
                    self.column_position = 0;
                } else {
                    self.column_position += 1;
                }
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }
}

use core::fmt;
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}



#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_text::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    WRITER.lock().write_fmt(args).unwrap();
}

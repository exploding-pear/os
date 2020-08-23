#![allow(dead_code)]
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
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

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightGray, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

pub struct Writer {
    // keeps track of the current position in the last row
    column_position: usize,
    // current foreground and background colors
    color_code: ColorCode,
    // reference to the VGA buffer. the 'static lifetime means live for whole program
    buffer: &'static mut Buffer,
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl Writer {
    // iterates over each byte in string and writes each one
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline case: write byte
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not ASCII or newline case: print a square
                _ => self.write_byte(0xfe),
            }
        }
    }
    // writes a byte to VGA buffer
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                // newline when current line full
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }
    fn new_line(&mut self) {
        // iterating over all screen characters
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                // moving each character 1 row up
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

// enabling copy sematincs for the type to make it printable and comparable
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// ensure struct is laid out exactly like a C struct to guarantee correct field ordering
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// ensuring that struct has same memory layout as single field
#[repr(transparent)]
struct Buffer {
    // using a volatile type to prevent any optimized reads/writes
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// enabling copy sematincs for the type to make it printable and comparable
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode {
    // creates the binary color code give a foreground and bacground Color
    fn new(foreground: Color, background: Color) -> ColorCode {
        // background color is first 4 bits, foreground color is last 4 bits
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// enabling copy sematincs for the type to make it printable and comparable
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// each enum variant stored as a unsigned 8bit integer
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    // iterating over each character in the string
    for (i, c) in s.chars().enumerate() {
        // locking the writer and getting the first character on the line
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        // assert that the character read matches the character in the string
        assert_eq!(char::from(screen_char.ascii_character), c)
    }
}

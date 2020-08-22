#[allow(dead_code)]

use volatile::Volatile;
use core::fmt;

pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer)},
    };
    writer.write_byte(b'H');
    writer.write_string("ello! ");
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
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
                    color_code: color_code,
                });
                self.column_position += 1;
            }
        }
    }
    fn new_line(&mut self) {/* TODO */}
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
    White = 15
}
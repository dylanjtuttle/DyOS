use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

// Implement a print! macro (mostly stolen from the real one)
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

// Implement a println! macro (mostly stolen from the real one)
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// Actually do the writing
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

// Create a nice interface for writing
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightGreen, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// Define an enum for colors, stored as u8 ints
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

// ColorCode type, defines the foreground and background colors
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        // Combine the foreground and background color into a single byte
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// ScreenChar struct to hold the character to be written and its ColorCode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode
}

// Defining the size of the VGA buffer
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// Creating a holder for the VGA buffer
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// Writer struct, holds the position on the screen of the character, its ColorCode, and a reference to the VGA buffer
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

// Writer type
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),  // If the byte is a newline, make a new line
            byte => {
                // Wrap text around the end of the buffer
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                
                // Define where the byte is going to be written
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                // Define the ColorCode of the byte to be written
                let color_code = self.color_code;

                // Write the byte to the buffer
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });

                // Ensure bytes don't write over each other by moving to the next column position
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        // Loop through the bytes and print them
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not a printable ASCII character
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        // Loop through all printed characters
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                // Move each character up one row
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        // Clear the row that has become cleared
        self.clear_row(BUFFER_HEIGHT - 1);
        // Set the column for writing back at the left side of the buffer
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        // Define a blank character
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        // Replace all characters on the given line with blank characters
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

// Write type
impl fmt::Write for Writer {
    // Write the given string
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
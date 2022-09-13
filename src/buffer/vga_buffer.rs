#[allow(dead_code)] //  disable unused variant warnings

use super::terminal_buffer::{TERMINAL_WRITER_1, TERMINAL_WRITER_2};
use super::keyboard::{SWITCH, IF_TAB};
use crate::terminal::SwitchState;
use crate::terminal::terminal1::PRINTING_1;
use crate::terminal::terminal2::PRINTING_2;
use crate::terminal::terminal1::EDITING_1;

pub static mut INITIAL: bool = false;
pub static mut IF_ESC: bool = false;
pub static mut EDIT_UP: bool = false;
pub static mut EDIT_DOWN: bool = false;

#[derive(Debug, Clone, Copy, PartialEq, Eq)] // we enable copy semantics for the type and make it printable and comparable
#[repr(u8)] // attribute each enum variant is stored as an u8
pub enum Color {    // enum the color printing on the screen
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);   // the byte represents the foreground & background color
impl ColorCode {
    fn new (foreground: Color, background: Color) -> ColorCode {
        ColorCode( (background as u8) << 4 | (foreground as u8) )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {     // the whole 2 bytes format printing on a signle grid
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;    // define the size of screen
const BUFFER_WIDTH: usize = 80;

use volatile::Volatile;
#[repr(transparent)]
pub struct Buffer {  // the whole screen buffer with size of 80 * 25
    pub chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer { // write something on a buffer with specified color
    cursor_x: usize,
    cursor_y: usize,
    cursor_toggle: bool,
    color_code: ColorCode,
    color_title: ColorCode,
    pub buffer: &'static mut Buffer, // the 'static lifetime specifies that the reference is valid for the whole program run time
}
impl Writer {
    pub fn write_byte (&mut self, byte: u8) {   // write an single byte
        match byte {
            0x01 => self.cursor_blink(),                    // for cursor
            0x02 => {                                       // for 'clear' command
                for i in 1..BUFFER_HEIGHT {
                    self.clear_row(i);
                }
                self.cursor_x = 0;
                self.cursor_y = 1;
            },
            0x03 => {                                       // for 'edit' command
                unsafe {
                    use crate::terminal::terminal1::SHIFT_OFFSET_1;
                    let total = self.cursor_y - SHIFT_OFFSET_1 - 2;
                    for _ in 0..total {
                        self.upper_shift();
                        self.cursor_y -= 1;
                    }
                }
            },
            0x04 => self.line_up(),
            0x05 => self.line_down(),
            0x08 => self.backspace(),                       // backspace keypress
            0x09 => self.switch(),                          // for tab keypress
            0x1b => {                                       // for esc, interrupt the command
                unsafe {
                    IF_ESC = true;
                }
            },
            b'\n' => self.new_line(),                       // newline when printing '\n'
            byte => {
                if self.cursor_x >= BUFFER_WIDTH {          // newline when typping at the right side
                    self.cursor_x = 0;
                    
                    if self.cursor_y == BUFFER_HEIGHT - 1 { // reach the bottom of the VGA
                        self.upper_shift();
                    } else {
                        self.cursor_y += 1;
                    }
                }

                let row = self.cursor_y;
                let col = self.cursor_x;

                if row == 0 {                                       // print the title
                    let color_code = self.color_title;
                    self.buffer.chars[row][col].write(ScreenChar {
                        ascii_character: byte,
                        color_code,
                    });                
                } else {                                            // not title
                    let color_code = self.color_code;
                    self.buffer.chars[row][col].write(ScreenChar {
                        ascii_character: byte,
                        color_code,
                    });
                }

                self.cursor_x += 1;
            }
        }
    }

    fn switch(&mut self) {
        unsafe {
            if SWITCH == SwitchState::Terminal1 {       // already switched to terminal1
                let buffer1 = &mut *(0xb9000 as *mut Buffer);
                for row in 0..BUFFER_HEIGHT {
                    for col in 0..BUFFER_WIDTH {
                        let character = buffer1.chars[row][col].read();
                        self.buffer.chars[row][col].write(character);
                    }
                }
                let color_code = self.color_title;
                self.buffer.chars[0][10].write(ScreenChar {
                    ascii_character: b'1',
                    color_code,
                });
            } else {                                    // already switched to terminal2
                let buffer2 = &mut *(0xba000 as *mut Buffer);
                for row in 0..BUFFER_HEIGHT {
                    for col in 0..BUFFER_WIDTH {
                        let character = buffer2.chars[row][col].read();
                        self.buffer.chars[row][col].write(character);
                    }
                }
                let color_code = self.color_title;
                self.buffer.chars[0][10].write(ScreenChar {
                    ascii_character: b'2',
                    color_code,
                });
            }
        }
    }

    fn new_line(&mut self) {
        if self.cursor_x < BUFFER_WIDTH {                                   // erase the cursor
            let color_code = ColorCode::new(Color::Yellow, Color::Black);
            let row = self.cursor_y;
            let col = self.cursor_x;
            self.buffer.chars[row][col].write(ScreenChar {
                ascii_character: 0x00,
                color_code,
            });
        }
    
        self.cursor_x = 0;
        if self.cursor_y == BUFFER_HEIGHT - 1 {     // reach the bottom of the VGA
            self.upper_shift();
        } else {
            unsafe {
                if EDITING_1 {                        // it's possible to move things out of the screen at bottom, need to be fixed
                    let mut row = BUFFER_HEIGHT - 2;
                    while row >= self.cursor_y {
                        for col in 0..BUFFER_WIDTH {
                            let character = self.buffer.chars[row][col].read();
                            self.buffer.chars[row + 1][col].write(character);
                        }
                        row -= 1;
                    }
                    self.clear_row(self.cursor_y + 1);
                }
            }            

            self.cursor_y += 1;
        }
    }

    fn upper_shift(&mut self) {    // print when reaching the bottom, the whole terminal should shift upper
        for row in 2..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.cursor_x = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: 0x00,
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    fn backspace(&mut self) {
        let character = ScreenChar {
            ascii_character: 0x00,
            color_code: ColorCode::new(Color::Black, Color::Black),
        };
        unsafe {
            /* avoid backspacing when reach specific position */
            if EDITING_1 == false {                               // command line mode
                let arrow_char = self.buffer.chars[self.cursor_y][self.cursor_x - 1].read(); // the left char is '>'
                let arrow_char = arrow_char.ascii_character;
                if arrow_char == b'>' {
                    return;
                }
            } else {                                            // text edit mode
                /* backspace to the last character that is not ' ', these block of code only fit the situation that the line is continuous. */
                if self.cursor_x == 0 && self.cursor_y == 2 {           // reach the first char in edit area.
                    return;
                }
            }

            /* move the cursor */
            if self.cursor_x == 0 {                         // reach the left
                let mut flag = false;
                for i in 0..BUFFER_WIDTH {
                    let c = self.buffer.chars[self.cursor_y - 1][i].read();
                    let c = c.ascii_character;
                    if !(c>=b' '&& c<=b'~') {
                        let character2 = ScreenChar {
                            ascii_character: 0x00,
                            color_code: ColorCode::new(Color::Yellow, Color::Black),
                        };
                        self.buffer.chars[self.cursor_y][self.cursor_x].write(character2);
        
                        self.cursor_x = i;
                        self.cursor_y -= 1;
                        flag = true;
                        break;
                    }
                }
                if flag == false {
                    self.cursor_y = self.cursor_y - 1;
                    self.cursor_x = BUFFER_WIDTH - 1;
                }
            } else {
                self.cursor_x = self.cursor_x - 1;
            }
            /* erase the possible older cursor the screen */
            let row = self.cursor_y;
            let col = self.cursor_x;
            self.buffer.chars[row][col].write(character);
            if col == BUFFER_WIDTH - 1 {
                self.buffer.chars[row + 1][0].write(character);
            } else {
                self.buffer.chars[row][col + 1].write(character);
            }

            /* if the first line below the cursor is empty, shift up all the lines below */
            if self.cursor_y == BUFFER_HEIGHT - 1 {
                return;
            }
            let mut flag = false;
            for col in 0..BUFFER_WIDTH {
                let c = self.buffer.chars[self.cursor_y + 1][col].read();
                if c.ascii_character != 0x00 {
                    flag = true;
                }
            }
            if flag == false {
                for row in self.cursor_y+1..BUFFER_HEIGHT-1 {
                    for col in 0..BUFFER_WIDTH {
                        let c = self.buffer.chars[row + 1][col].read();
                        self.buffer.chars[row][col].write(c);
                    }
                }
                
                let character = ScreenChar {
                    ascii_character: 0x00,
                    color_code: ColorCode::new(Color::Yellow, Color::Black),
                };
                for col in 0..BUFFER_WIDTH {
                    self.buffer.chars[BUFFER_HEIGHT - 1][col].write(character);
                }
            }
        }
    }

    fn line_up(&mut self) {
        unsafe { EDIT_UP = true; }
        unsafe {
            if !EDITING_1 {       // only apply for the file editing mode
                return;
            }
        }        
        if self.cursor_y == 2 {   // reach the fist line of the text area
            return;
        }

        for i in 0..BUFFER_WIDTH {
            let c = self.buffer.chars[self.cursor_y - 1][i].read();
            let c = c.ascii_character;
            if !(c>=b' '&& c<=b'~') {
                let character2 = ScreenChar {
                    ascii_character: 0x00,
                    color_code: ColorCode::new(Color::Yellow, Color::Black),
                };
                self.buffer.chars[self.cursor_y][self.cursor_x].write(character2);

                self.cursor_x = i;
                self.cursor_y -= 1;
                break;
            }
        }
    }

    fn line_down(&mut self) {
        unsafe { EDIT_DOWN = true; }
        unsafe {
            if !EDITING_1 {       // only apply for the file editing mode
                return;
            }
        }        
        if self.cursor_y == BUFFER_HEIGHT - 1 {   // reach the last line of the text area
            return;
        }

        for i in 0..BUFFER_WIDTH {
            let c = self.buffer.chars[self.cursor_y + 1][i].read();
            let c = c.ascii_character;
            if !(c>=b' '&& c<=b'~') {
                let character2 = ScreenChar {
                    ascii_character: 0x00,
                    color_code: ColorCode::new(Color::Yellow, Color::Black),
                };
                self.buffer.chars[self.cursor_y][self.cursor_x].write(character2);

                self.cursor_x = i;
                self.cursor_y += 1;
                break;
            }
        }
    }

    fn cursor_blink(&mut self) {
        let character1 = ScreenChar {   // cursor state1
            ascii_character: b'_',
            color_code: ColorCode::new(Color::Yellow, Color::Black),
        };
        let character2 = ScreenChar {   // cursor state2
            ascii_character: 0x00,
            color_code: ColorCode::new(Color::Yellow, Color::Black),
        };
        
        let mut row = self.cursor_y;
        let mut col = self.cursor_x;

        if col == BUFFER_WIDTH {        // reach the right side
            col = 0;
            if row != BUFFER_HEIGHT - 1 {
                row += 1;
            } else {                    // in the right bottom corner, the cursor should not blink any more.
                return;
            }
        }
    
        if self.cursor_toggle {
            self.buffer.chars[row][col].write(character1);
            self.cursor_toggle = false;
        } else {
            self.buffer.chars[row][col].write(character2);
            self.cursor_toggle = true;
        }
    }

    pub fn write_string(&mut self, s: &str) {   // write a whole string
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | 0x02 | 0x03 | 0x08 | 0x09 | 0x1b | 0xff | b'\n' => self.write_byte(byte),
                0x04 | 0x05 => self.write_byte(byte),           // arrow up and arrow down
                0x01 => self.write_byte(0x01),                  // cursor driven by timer
                // 0x02 => self.write_byte(0x02),                  // for 'clear' command
                // 0x1b => self.write_byte(5),
                _ => self.write_byte(0xfe),                     // not part of printable ASCII range, print as '■'
            }
        }
    }
}
use core::fmt;
impl fmt::Write for Writer {
    fn write_str(&mut self, s:&str) -> fmt::Result {    // wrap Writer::write_string in Writer::write_str
        self.write_string(s);
        Ok(())
    }
}

use lazy_static::lazy_static;
use spin::Mutex;
/*
While the static are initialized at compile time, normal variables are initialized at run time.
Use lazy statics to define a lazily initialized static that initialization happens at runtime by using macro 'lazy_static!'
*/
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {  
        // Use mutable variable 'Writer' and add a mutex lock on it to make it safe from data race
        cursor_x: 0,
        cursor_y: 0,
        cursor_toggle: true,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        color_title: ColorCode::new(Color::Black, Color::White),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::buffer::vga_buffer::_print(format_args!($($arg)*))) // the same as std print!
}

#[macro_export]
macro_rules! println {      // use a macro to create a safa and convinient interface to the outside
    // Please write the code 'use crate::println' when you want to use the macro.
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)] // hide the function from the generated documentation
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {     // use a clousure and execute it an interrupt-free environment
        unsafe {
            if INITIAL == false {                                           // initial 'Welcome to derbo OS'
                WRITER.lock().write_fmt(args).unwrap();

                TERMINAL_WRITER_1.lock().write_fmt(args).unwrap();
                TERMINAL_WRITER_2.lock().write_fmt(args).unwrap();

                let cur_x = WRITER.lock().cursor_x;
                let cur_y = WRITER.lock().cursor_y;
                
                TERMINAL_WRITER_1.lock().cursor_x = cur_x;
                TERMINAL_WRITER_1.lock().cursor_y = cur_y;
                TERMINAL_WRITER_2.lock().cursor_x = cur_x;
                TERMINAL_WRITER_2.lock().cursor_y = cur_y;
            } else {
                // TODO: the print of background task should not show in the VGA, and just write into TERMINAL_WRITER

                /* firstly, duplicate the current TERMINAL_WRITER !!!only cursor!!! to WRITER */
            
                if IF_TAB {                                     // tab
                    if SWITCH == SwitchState::Terminal1 {                   // already switched to terminal1
                        WRITER.lock().cursor_x = TERMINAL_WRITER_1.lock().cursor_x;
                        WRITER.lock().cursor_y = TERMINAL_WRITER_1.lock().cursor_y;
                    } else {                                                // already switched to terminal2
                        WRITER.lock().cursor_x = TERMINAL_WRITER_2.lock().cursor_x;
                        WRITER.lock().cursor_y = TERMINAL_WRITER_2.lock().cursor_y;
                    }
                    WRITER.lock().write_fmt(args).unwrap();     // duplicate all chars from terminal buffer to VGA
                }
                else {
                    /* secondly, write chars in the VGA */
                    if SWITCH == SwitchState::Terminal1 && PRINTING_1 && PRINTING_2 {           // I won't do this manually

                    } else if SWITCH == SwitchState::Terminal1 && PRINTING_1 && !PRINTING_2 {
                        WRITER.lock().write_fmt(args).unwrap();
                        TERMINAL_WRITER_1.lock().write_fmt(args).unwrap();
                    } else if SWITCH == SwitchState::Terminal1 && !PRINTING_1 && PRINTING_2 {
                        TERMINAL_WRITER_2.lock().write_fmt(args).unwrap();
                    } else if SWITCH == SwitchState::Terminal1 && !PRINTING_1 && !PRINTING_2 {  // !!!!!
                        WRITER.lock().write_fmt(args).unwrap();
                        TERMINAL_WRITER_1.lock().write_fmt(args).unwrap();
                    } else if SWITCH == SwitchState::Terminal2 && PRINTING_1 && PRINTING_2 {    // I won't do this manually
                        
                    } else if SWITCH == SwitchState::Terminal2 && PRINTING_1 && !PRINTING_2 {
                        TERMINAL_WRITER_1.lock().write_fmt(args).unwrap();
                    } else if SWITCH == SwitchState::Terminal2 && !PRINTING_1 && PRINTING_2 {
                        WRITER.lock().write_fmt(args).unwrap();
                        TERMINAL_WRITER_2.lock().write_fmt(args).unwrap();
                    } else if SWITCH == SwitchState::Terminal2 && !PRINTING_1 && !PRINTING_2 {  // !!!!! 
                        WRITER.lock().write_fmt(args).unwrap();
                        TERMINAL_WRITER_2.lock().write_fmt(args).unwrap();
                    }
                }
            }
        }
    });
}
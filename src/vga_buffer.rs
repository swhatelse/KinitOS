use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

const SCREEN_HEIGHT: usize = 25;
const SCREEN_WIDTH: usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]    
pub enum FontColor {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xa,
    LightCyan = 0xb,
    LightRed = 0xc,
    Pink = 0xd,
    Yellow = 0xe,
    White = 0xf
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BackgroundColor {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
struct ColorCode{
    code : u8
}

#[allow(dead_code)]
impl ColorCode{
    fn new(ft_color: FontColor, bg_color: BackgroundColor) -> Self{
         Self {code: (ft_color as u8) | ((bg_color as u8) << 4) }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct VGAChar{
    char: u8,
    color_code: ColorCode
}

#[repr(transparent)]
struct VGABuffer{
    data: [[Volatile<VGAChar>; SCREEN_WIDTH]; SCREEN_HEIGHT]
}

pub struct Writer{
    color: ColorCode,
    buffer: &'static mut VGABuffer,
    colpos: usize,
    rowpos: usize
}

impl Writer{
    pub fn new() -> Self{
        Self { color: ColorCode::new(FontColor::LightGray, BackgroundColor::Black),
               buffer: unsafe {&mut *(0xb8000 as *mut VGABuffer) },
               colpos: 0,
               rowpos:0
        }
    }
    
    #[allow(dead_code)]
    pub fn set_color(&mut self, ft_color: FontColor, bg_color: BackgroundColor){
        self.color = ColorCode::new( ft_color, bg_color );
    }

    fn new_line(&mut self){
        self.rowpos += 1;
        self.colpos = 0;

        if self.rowpos >= SCREEN_HEIGHT {
            self.rowpos -= 1;
            self.shift_up();
        }
    }

    /**
     * Shift the VGA buffer upward.
     */
    fn shift_up(&mut self){
        for i in 1..SCREEN_HEIGHT {
            for j in 0..SCREEN_WIDTH {
                self.buffer.data[i - 1][j].write( self.buffer.data[i][j].read() );
            }
        }

        for i in 0..SCREEN_WIDTH {
            self.buffer.data[SCREEN_HEIGHT - 1][i].write( VGAChar{ char: b' ', color_code: self.color } );
        }
    }
    
    /**
     * Print a string in the VGA buffer at a given position on the screen.
     * @param[in] s String to print
     */
    pub fn print(&mut self, s: &str){
        for byte in s.bytes() {
            match byte {
                b'\n' | 0x20..=0x7e  => self.print_char(byte),
                _=> self.print_char(0xfe)
            }
        }
    }

    /**
     * Print a character in the VGA buffer at a given position on the screen.
     * @param[in] c Character to print
     */
    fn print_char(&mut self, c: u8) {
        match c {
            b'\n' => self.new_line(),
            _ => {
                self.buffer.data[self.rowpos][self.colpos].write(VGAChar { char: c, color_code: self.color });
                if self.colpos >= SCREEN_WIDTH - 1 {
                    self.new_line();
                }
                else{
                    self.colpos += 1;
                }
            }
        }
    }
}

// Implementation to support formating macro
impl fmt::Write for Writer{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => ($crate::vga_buffer::_print(format_args!($($args)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($args:tt)*) => ($crate::print!("{}\n", format_args!($($args)*)));
}

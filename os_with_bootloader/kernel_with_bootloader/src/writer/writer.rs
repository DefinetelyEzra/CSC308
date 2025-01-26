mod constants;
use core::{
    fmt::{self, Write},
    ptr,
};
use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use constants::font_constants;
use constants::font_constants::{BACKUP_CHAR, CHAR_RASTER_HEIGHT, FONT_WEIGHT};
use noto_sans_mono_bitmap::{get_raster, RasterizedChar};

/// Additional vertical space between lines 
const LINE_SPACING: usize = 2;

/// Additional horizontal space between characters. 
const LETTER_SPACING: usize = 0;

/// Padding from the border. Prevent that font is too close to border. 
const BORDER_PADDING: usize = 1;

/// Returns the raster of the given char or the raster of [font_constants::BACKUP_CHAR].
fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Should get raster of backup char."))
}

/// Allows logging text to a pixel-based framebuffer.
pub struct FrameBufferWriter {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
    color: [u8; 3], // RGB color for text
}

impl FrameBufferWriter {
    /// Creates a new logger that uses the given framebuffer
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut logger = Self {
            framebuffer,
            info,
            x_pos: BORDER_PADDING,
            y_pos: BORDER_PADDING,
            color: [255, 255, 255], 
        };
        logger.clear();
        logger
    }

    fn newline(&mut self) {
        self.y_pos += CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        if self.y_pos >= self.height() {
            self.scroll_screen();
            self.y_pos = self.height() - CHAR_RASTER_HEIGHT.val() - LINE_SPACING;
        }
        self.carriage_return();
    }

    fn carriage_return(&mut self) {
        self.x_pos = BORDER_PADDING;
    }

    /// Erases all text on the screen. Resets self.x_pos and self.y_pos.
    pub fn clear(&mut self) {
        self.x_pos = BORDER_PADDING;
        self.y_pos = BORDER_PADDING;
        self.framebuffer.fill(0);
    }

    fn width(&self) -> usize {
        self.info.width
    }

    fn height(&self) -> usize {
        self.info.height
    }

    /// Scrolls the screen up by one line.
    fn scroll_screen(&mut self) {
        let row_size = self.info.stride * CHAR_RASTER_HEIGHT.val();
        self.framebuffer.copy_within(row_size.., 0);
        let clear_start = self.framebuffer.len() - row_size;
        self.framebuffer[clear_start..].fill(0);
    }

    /// Writes a single char to the framebuffer. Takes care of special control characters and escape sequences.
    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            '\t' => {
                self.x_pos += 4 * CHAR_RASTER_HEIGHT.val();
                if self.x_pos >= self.width() {
                    self.newline();
                }
            }
            '\\c' => {
                self.color = [0, 0, 255]; // Turns text Blue hopefully
            }
            _ => {
                let new_xpos = self.x_pos + CHAR_RASTER_HEIGHT.val();
                if new_xpos >= self.width() {
                    self.newline();
                }
                if self.y_pos + CHAR_RASTER_HEIGHT.val() >= self.height() {
                    self.scroll_screen();
                }
                self.write_rendered_char(get_char_raster(c));
            }
        }
    }

    /// Prints a rendered char into the framebuffer. Updates self.x_pos.
    fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, &byte) in row.iter().enumerate() {
                self.write_pixel(self.x_pos + x, self.y_pos + y, byte);
            }
        }
        self.x_pos += rendered_char.width() + LETTER_SPACING;
    }

    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        if x >= self.width() || y >= self.height() {
            return; // Error handling
        }

        let pixel_offset = y * self.info.stride + x;
        let color = match self.info.pixel_format {
            PixelFormat::Rgb => [
                self.color[0] * intensity / 255,
                self.color[1] * intensity / 255,
                self.color[2] * intensity / 255,
                0,
            ],
            PixelFormat::Bgr => [
                self.color[2] * intensity / 255,
                self.color[1] * intensity / 255,
                self.color[0] * intensity / 255,
                0,
            ],
            PixelFormat::U8 => [if intensity > 200 { 0xf } else { 0 }, 0, 0, 0],
            other => {
                self.info.pixel_format = PixelFormat::Rgb;
                panic!("Pixel format {:?} not supported in logger", other);
            }
        };

        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);
        let _ = unsafe { ptr::read_volatile(&self.framebuffer[byte_offset]) };
    }
}

unsafe impl Send for FrameBufferWriter {}
unsafe impl Sync for FrameBufferWriter {}

impl Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($writer:expr, $($arg:tt)*) => {
        $writer.write_fmt(format_args!($($arg)*)).unwrap();
    };
}

use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicU16, Ordering};

// Define static variables for viewport size
static VIEWPORT_WIDTH: AtomicU16 = AtomicU16::new(80);
static VIEWPORT_HEIGHT: AtomicU16 = AtomicU16::new(24);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attribute {
    Reset = 0,
    Bright = 1,
    Dim = 2,
    Underscore = 4,
    Blink = 5,
    Reverse = 7,
    Hidden = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxStyle {
    Single,
    Double,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxChar {
    Horizontal(BoxStyle),
    Vertical(BoxStyle),
    TopLeft(BoxStyle),
    TopRight(BoxStyle),
    BottomLeft(BoxStyle),
    BottomRight(BoxStyle),
    VerticalLeft(BoxStyle),
    VerticalRight(BoxStyle),
    HorizontalDown(BoxStyle),
    HorizontalUp(BoxStyle),
    VerticalHorizontal(BoxStyle),
}

impl BoxChar {
    pub fn to_char(&self) -> char {
        match self {
            BoxChar::Horizontal(BoxStyle::Single) => '─',
            BoxChar::Vertical(BoxStyle::Single) => '│',
            BoxChar::TopLeft(BoxStyle::Single) => '┌',
            BoxChar::TopRight(BoxStyle::Single) => '┐',
            BoxChar::BottomLeft(BoxStyle::Single) => '└',
            BoxChar::BottomRight(BoxStyle::Single) => '┘',
            BoxChar::VerticalLeft(BoxStyle::Single) => '├',
            BoxChar::VerticalRight(BoxStyle::Single) => '┤',
            BoxChar::HorizontalDown(BoxStyle::Single) => '┬',
            BoxChar::HorizontalUp(BoxStyle::Single) => '┴',
            BoxChar::VerticalHorizontal(BoxStyle::Single) => '┼',
            BoxChar::Horizontal(BoxStyle::Double) => '═',
            BoxChar::Vertical(BoxStyle::Double) => '║',
            BoxChar::TopLeft(BoxStyle::Double) => '╔',
            BoxChar::TopRight(BoxStyle::Double) => '╗',
            BoxChar::BottomLeft(BoxStyle::Double) => '╚',
            BoxChar::BottomRight(BoxStyle::Double) => '╝',
            BoxChar::VerticalLeft(BoxStyle::Double) => '╠',
            BoxChar::VerticalRight(BoxStyle::Double) => '╣',
            BoxChar::HorizontalDown(BoxStyle::Double) => '╦',
            BoxChar::HorizontalUp(BoxStyle::Double) => '╩',
            BoxChar::VerticalHorizontal(BoxStyle::Double) => '╬',
        }
    }
}

/// Sets the foreground color for subsequent text output.
///
/// # Arguments
///
/// * `color` - The color to set for the foreground.
///
/// # Errors
///
/// Returns an `io::Error` if writing to stdout fails.
pub fn set_foreground_color(color: Color) -> io::Result<()> {
    write!(io::stdout(), "\x1B[3{}m", color as u8)?;
    io::stdout().flush()
}

/// Sets the background color for subsequent text output.
///
/// # Arguments
///
/// * `color` - The color to set for the background.
///
/// # Errors
///
/// Returns an `io::Error` if writing to stdout fails.
pub fn set_background_color(color: Color) -> io::Result<()> {
    write!(io::stdout(), "\x1B[4{}m", color as u8)?;
    io::stdout().flush()
}

/// Resets all text formatting to default.
///
/// # Errors
///
/// Returns an `io::Error` if writing to stdout fails.
pub fn reset_color() -> io::Result<()> {
    write!(io::stdout(), "\x1B[0m")?;
    io::stdout().flush()
}

/// Moves the cursor to the specified (x, y) coordinates in the terminal.
///
/// # Arguments
///
/// * `x` - The column position (1-based index)
/// * `y` - The row position (1-based index)
///
/// # Errors
///
/// Returns an `io::Error` if writing to stdout fails.

pub fn move_cursor_to(x: u16, y: u16) -> io::Result<()> {
    let (viewport_width, viewport_height) = get_viewport();
    if x > viewport_width || y > viewport_height {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Cursor position is outside viewport"));
    }
    write!(io::stdout(), "\x1B[{};{}H", y, x)?;
    io::stdout().flush()
}



/// Prints a single character to the terminal and immediately flushes the output.
///
/// # Arguments
///
/// * `ch` - The character to print
///
/// # Errors
///
/// Returns an `io::Error` if writing to stdout fails.
pub fn put_char(ch: char) -> io::Result<()> {
    write!(io::stdout(), "{}", ch)?;
    io::stdout().flush()
}

/// Clears the entire screen and moves the cursor to position (1, 1).
///
/// # Errors
///
/// Returns an `io::Error` if writing to stdout fails.
pub fn clear_screen() -> io::Result<()> {
    write!(io::stdout(), "\x1B[2J")?;
    move_cursor_to(1, 1)
}

/// Waits for the specified number of seconds.
///
/// # Arguments
///
/// * `seconds` - The number of seconds to wait.
pub fn wait_for_seconds(seconds: u64) {
    thread::sleep(Duration::from_secs(seconds));
}

/// Waits for the specified number of milliseconds.
///
/// # Arguments
///
/// * `milliseconds` - The number of milliseconds to wait.
pub fn wait_for_millis(milliseconds: u64) {
    thread::sleep(Duration::from_millis(milliseconds));
}

/// Waits for the specified number of microseconds.
///
/// # Arguments
///
/// * `microseconds` - The number of microseconds to wait.
pub fn wait_for_micros(microseconds: u64) {
    thread::sleep(Duration::from_micros(microseconds));
}

#[deprecated(since = "0.5.0", note = "Use read_key instead, which provides the same functionality")]
pub fn read_char() -> io::Result<char> {
    read_key()
}


/// Reads a line from the standard input, trimming whitespace.
///
/// # Returns
///
/// Returns a `Result` containing the read line as a `String`, or an `io::Error` if the read fails.
pub fn read_line() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

/// Prints the given text to the standard output and flushes the buffer.
///
/// # Arguments
///
/// * `text` - The text to print.
///
/// # Returns
///
/// Returns `Ok(())` if the print and flush succeed, or an `io::Error` if either operation fails.
pub fn print(text: &str) -> io::Result<()> {
    io::stdout().write_all(text.as_bytes())?;
    io::stdout().flush()
}

/// Prints the given text to the standard output, followed by a newline, and flushes the buffer.
///
/// # Arguments
///
/// * `text` - The text to print.
///
/// # Returns
///
/// Returns `Ok(())` if the print and flush succeed, or an `io::Error` if either operation fails.
pub fn println(text: &str) -> io::Result<()> {
    print(&(text.to_owned() + "\n"))
}

/// Saves the current cursor location.
///
/// # Errors
///
/// Returns an `io::Error` if writing to stdout fails.
pub fn save_cursor_location() -> io::Result<()> {
    write!(io::stdout(), "\x1B[s")?;
    io::stdout().flush()
}

/// Restores the cursor to the previously saved location.
///
/// # Errors
///
/// Returns an `io::Error` if writing to stdout fails.
pub fn restore_cursor_location() -> io::Result<()> {
    write!(io::stdout(), "\x1B[u")?;
    io::stdout().flush()
}

/// Makes the cursor visible.
///
/// # Errors
///
/// Returns an `io::Error` if writing to stdout fails.
pub fn show_cursor() -> io::Result<()> {
    write!(io::stdout(), "\x1B[?25h")?;
    io::stdout().flush()
}

/// Hides the cursor.
///
/// # Errors
///
/// Returns an `io::Error` if writing to stdout fails.
pub fn hide_cursor() -> io::Result<()> {
    write!(io::stdout(), "\x1B[?25l")?;
    io::stdout().flush()
}

/// Reads a key press from the standard input.
///
/// This function reads a line from the input, returns the first character,
/// and clears any remaining input in the buffer.
///
/// # Returns
///
/// Returns a `Result` containing the first character of the input line (or '\0' if the line was empty),
/// or an `io::Error` if the read fails.
///
/// # Examples
///
/// ```
/// use rpian_terminal::read_key;
///
/// fn main() -> std::io::Result<()> {
///     println!("Press a key and then Enter:");
///     let key = read_key()?;
///     println!("You pressed: {}", key);
///     Ok(())
/// }
/// ```
pub fn read_key() -> io::Result<char> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    // Get the first character
    let first_char = input.chars().next().unwrap_or('\0');
    
    // Clear the rest of the input buffer
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    stdin.read_line(&mut String::new())?;
    stdout.flush()?;
    
    Ok(first_char)
}


/// Clears from the cursor to the end of the line.
///
/// # Errors
///
/// Returns an `io::Error` if writing to stdout fails.
pub fn clear_to_line_end() -> io::Result<()> {
    write!(io::stdout(), "\x1b[K")?;
    io::stdout().flush()
}

/// Clears from the cursor to the start of the line.
///
/// # Errors
///
/// Returns an `io::Error` if writing to stdout fails.
pub fn clear_to_line_start() -> io::Result<()> {
    write!(io::stdout(), "\x1b[1K")?;
    io::stdout().flush()
}

/// Clears the entire line.
///
/// # Errors
///
/// Returns an `io::Error` if writing to stdout fails.
pub fn clear_line() -> io::Result<()> {
    write!(io::stdout(), "\x1b[2K")?;
    io::stdout().flush()
}

/// Clears from the cursor to the start of the screen.
///
/// # Errors
///
/// Returns an `io::Error` if writing to stdout fails.
pub fn clear_to_screen_start() -> io::Result<()> {
    write!(io::stdout(), "\x1b[1J")?;
    io::stdout().flush()
}

/// Clears from the cursor to the end of the screen.
///
/// # Errors
///
/// Returns an `io::Error` if writing to stdout fails.
pub fn clear_to_screen_end() -> io::Result<()> {
    write!(io::stdout(), "\x1b[J")?;
    io::stdout().flush()
}

/// Sets the specified text attribute for subsequent text output.
///
/// # Arguments
///
/// * `attribute` - The attribute to set for the text.
///
/// # Errors
///
/// Returns an `io::Error` if writing to stdout fails.
pub fn set_attribute(attribute: Attribute) -> io::Result<()> {
    write!(io::stdout(), "\x1B[{}m", attribute as u8)?;
    io::stdout().flush()
}

/// Resets all text attributes to default.
///
/// # Errors
///
/// Returns an `io::Error` if writing to stdout fails.
pub fn reset_attributes() -> io::Result<()> {
    set_attribute(Attribute::Reset)
}

pub fn set_viewport(width: u16, height: u16) {
    VIEWPORT_WIDTH.store(width, Ordering::Relaxed);
    VIEWPORT_HEIGHT.store(height, Ordering::Relaxed);
}

/// Gets the current viewport size.
pub fn get_viewport() -> (u16, u16) {
    (
        VIEWPORT_WIDTH.load(Ordering::Relaxed),
        VIEWPORT_HEIGHT.load(Ordering::Relaxed),
    )
}

// Helper function to draw a single horizontal line with box characters
pub fn horizontal_line(box_char: BoxChar, x: u16, y: u16, length: u16) -> io::Result<()> {
    move_cursor_to(x, y)?;
    for _ in 0..length {
        write!(io::stdout(), "{}", box_char.to_char())?;
    }
    Ok(())
}

// Helper function to draw a single vertical line with box characters
pub fn vertical_line(box_char: BoxChar, x: u16, y: u16, length: u16) -> io::Result<()> {
    for i in 0..length {
        move_cursor_to(x, y + i)?;
        write!(io::stdout(), "{}", box_char.to_char())?;
    }
    Ok(())
}

/// Draws a diagonal line using the specified symbol.
///
/// # Arguments
///
/// * `symbol` - The character to use for drawing the line
/// * `x1` - The starting x-coordinate
/// * `y1` - The starting y-coordinate
/// * `x2` - The ending x-coordinate
/// * `y2` - The ending y-coordinate
///
/// # Errors
///
/// Returns an `io::Error` if writing to stdout fails or if the line extends beyond the viewport.
pub fn diagonal_line(symbol: char, x1: u16, y1: u16, x2: u16, y2: u16) -> io::Result<()> {
    let (viewport_width, viewport_height) = get_viewport();
    
    // Check if start and end points are within viewport
    if x1 > viewport_width || y1 > viewport_height || x2 > viewport_width || y2 > viewport_height {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Line coordinates are outside viewport"));
    }

    // Bresenham's line algorithm
    let dx = (x2 as i32 - x1 as i32).abs();
    let dy = -(y2 as i32 - y1 as i32).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut x = x1 as i32;
    let mut y = y1 as i32;

    loop {
        // Check if current point is within viewport
        if x >= 0 && x < viewport_width as i32 && y >= 0 && y < viewport_height as i32 {
            move_cursor_to(x as u16, y as u16)?;
            write!(io::stdout(), "{}", symbol)?;
        }

        if x == x2 as i32 && y == y2 as i32 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }

    io::stdout().flush()
}


pub fn draw_box(x: u16, y: u16, width: u16, height: u16, style: BoxStyle) -> io::Result<()> {
    let (viewport_width, viewport_height) = get_viewport();
    
    if x + width > viewport_width || y + height > viewport_height {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Box extends beyond viewport"));
    }

    horizontal_line(BoxChar::Horizontal(style), x + 1, y, width - 2)?;
    horizontal_line(BoxChar::Horizontal(style), x + 1, y + height - 1, width - 2)?;
    vertical_line(BoxChar::Vertical(style), x, y + 1, height - 2)?;
    vertical_line(BoxChar::Vertical(style), x + width - 1, y + 1, height - 2)?;

    // Draw corners
    move_cursor_to(x, y)?;
    put_char(BoxChar::TopLeft(style).to_char())?;
    
    move_cursor_to(x + width - 1, y)?;
    put_char(BoxChar::TopRight(style).to_char())?;
    
    move_cursor_to(x, y + height - 1)?;
    put_char(BoxChar::BottomLeft(style).to_char())?;
    
    move_cursor_to(x + width - 1, y + height - 1)?;
    put_char(BoxChar::BottomRight(style).to_char())?;

    io::stdout().flush()
}

use std::io::{self, Write, Read};
use std::thread;
use std::time::Duration;

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
pub fn move_cursor_to(x: u32, y: u32) -> io::Result<()> {
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

/// Reads a single character from the standard input.
///
/// # Returns
///
/// Returns a `Result` containing the read character, or an `io::Error` if the read fails.
pub fn read_char() -> io::Result<char> {
    let mut buffer = [0; 1];
    io::stdin().read_exact(&mut buffer)?;
    Ok(buffer[0] as char)
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

/// Reads a single key press from the standard input.
///
/// # Returns
///
/// Returns a `Result` containing the read character, or an `io::Error` if the read fails.
/// If the input is empty, it returns the null character '\0'.
pub fn read_key() -> io::Result<char> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.chars().next().unwrap_or('\0'))
}
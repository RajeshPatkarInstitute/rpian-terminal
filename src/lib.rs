use std::io::{self, Write};

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
pub fn teleport_to(x: u32, y: u32) -> io::Result<()> {
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
    teleport_to(1, 1)
}
//! rpian-terminal: A Rust library for terminal manipulation and drawing
//!
//! This library provides a set of functions for manipulating the terminal,
//! including cursor movement, color settings, and drawing various shapes.
use std::io::{self, Write};
use std::sync::atomic::{AtomicU16, Ordering};
use std::thread;
use std::time::Duration;

use error::{handle_boundary_error, handle_io_error};

pub mod error;
pub mod arrow;
pub mod braille;
pub mod chess;
pub mod emoji;
pub mod math;
pub mod rbox;
pub mod star;
pub mod triangle;
pub mod line;
pub mod circle;

// Define static variables for viewport size
static VIEWPORT_WIDTH: AtomicU16 = AtomicU16::new(80);
static VIEWPORT_HEIGHT: AtomicU16 = AtomicU16::new(24);

/// Represents the available colors for text and background.
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

/// Represents text attributes for styling.
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


/// Sets the foreground color for subsequent text output in the terminal.
pub fn set_foreground_color(color: Color) {
    print(&format!("\x1B[3{}m", color as u8));
}

/// Sets the background color for subsequent text output in the terminal.
pub fn set_background_color(color: Color) {
    print(&format!("\x1B[4{}m", color as u8));
}

/// Resets both the foreground and background colors to their default values.
pub fn reset_color() {
    print("\x1B[0m");
}

/// Moves the cursor to the specified position in the terminal.
pub fn move_cursor_to(x: u16, y: u16) {
    let (viewport_width, viewport_height) = get_viewport();
    if x > viewport_width || y > viewport_height {
        handle_boundary_error("Cursor position is outside viewport");
        return;
    }
    print(&format!("\x1B[{};{}H", y, x));
}

/// Clears the entire screen and moves the cursor to the top-left corner.
pub fn clear_screen() {
    print("\x1B[2J");
    move_cursor_to(1, 1);
}

/// Waits for the specified number of seconds.
pub fn wait_for_seconds(seconds: u64) {
    thread::sleep(Duration::from_secs(seconds));
}

/// Waits for the specified number of milliseconds.
pub fn wait_for_millis(milliseconds: u64) {
    thread::sleep(Duration::from_millis(milliseconds));
}

/// Waits for the specified number of microseconds.
pub fn wait_for_micros(microseconds: u64) {
    thread::sleep(Duration::from_micros(microseconds));
}

/// Reads a line from the standard input, trimming whitespace.
pub fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(e) => {
            handle_io_error(e);
            String::new()
        }
    }
}

/// Saves the current cursor location.
pub fn save_cursor_location() {
    print("\x1B[s");
}

/// Restores the cursor to the previously saved location.
pub fn restore_cursor_location() {
    print("\x1B[u");
}

/// Makes the cursor visible.
pub fn show_cursor() {
    print("\x1B[?25h");
}

/// Hides the cursor.
pub fn hide_cursor() {
    print("\x1B[?25l");
}

/// Reads a key press from the standard input.
pub fn read_key() -> char {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let first_char = input.chars().next().unwrap_or('\0');
            
            // Clear the rest of the input buffer
            if let Err(e) = io::stdin().read_line(&mut String::new()) {
                handle_io_error(e);
            }
            if let Err(e) = io::stdout().flush() {
                handle_io_error(e);
            }

            first_char
        },
        Err(e) => {
            handle_io_error(e);
            '\0'
        }
    }
}

/// Clears from the cursor to the end of the line.
pub fn clear_to_line_end() {
    print("\x1b[K");
}

/// Clears from the cursor to the start of the line.
pub fn clear_to_line_start() {
    print("\x1b[1K");
}

/// Clears the entire line.
pub fn clear_line() {
    print("\x1b[2K");
}

/// Clears from the cursor to the start of the screen.
pub fn clear_to_screen_start() {
    print("\x1b[1J");
}

/// Clears from the cursor to the end of the screen.
pub fn clear_to_screen_end() {
    print("\x1b[J");
}

/// Sets the specified text attribute for subsequent text output.
pub fn set_attribute(attribute: Attribute) {
    print(&format!("\x1B[{}m", attribute as u8));
}

/// Resets all text attributes to their default values.
pub fn reset_attributes() {
    set_attribute(Attribute::Reset);
}

/// Sets the viewport size.
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

/// Writes the given text to the standard output and flushes the buffer.
pub fn print(text: &str) {
    if let Err(e) = io::stdout().write_all(text.as_bytes()).and_then(|_| io::stdout().flush()) {
        handle_io_error(e);
    }
}

/// Writes the given text to the standard output, followed by a newline, and flushes the buffer.
pub fn println(text: &str) {
    print(&(text.to_owned() + "\n"));
}

/// Writes a single character to the standard output and flushes the buffer.
pub fn put_char(ch: char) {
    print(&ch.to_string());
}

// Re-export key types and functions from modules for easier access
pub use line::{Line, Direction, LineStyle};
pub use rbox::{BoxStyle, ShadeStyle};
pub use circle::CircleSymbol;

// You might want to add any new public functions or types here that are part of the main API
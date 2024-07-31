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
///
/// This function changes the color of the text that will be printed after it's called.
/// The color remains in effect until it is changed again or reset.
///
/// # Arguments
///
/// * `color` - A value of the `Color` enum representing the desired foreground color.
///
/// # Behavior
///
/// 1. Generates the appropriate ANSI escape sequence for the specified color.
/// 2. Writes this sequence to stdout using the `print` function.
/// 3. All text output after this function call will use the new foreground color.
///
/// # Error Handling
///
/// This function does not return a Result. Any I/O errors that occur during the
/// write operation are handled by the current error handler (see `set_error_handler`).
/// By default, errors are logged to stderr.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use rpian_terminal::{set_foreground_color, println, Color, reset_color};
///
/// set_foreground_color(Color::Red);
/// println("This text will be red.");
/// set_foreground_color(Color::Blue);
/// println("This text will be blue.");
/// reset_color();
/// println("This text will be in the default color.");
/// ```
///
/// Combining with background color:
///
/// ```
/// use rpian_terminal::{set_foreground_color, set_background_color, println, Color, reset_color};
///
/// set_foreground_color(Color::White);
/// set_background_color(Color::Blue);
/// println("White text on blue background.");
/// reset_color();
/// ```
///
/// # Note
///
/// - The actual appearance of colors may vary depending on the terminal emulator being used.
/// - Some terminals may not support all colors or may display them differently.
/// - It's a good practice to call `reset_color()` after you're done with colored output
///   to return to the default terminal colors.
/// - This function does not affect previously output text; it only changes the color
///   for subsequent output.
///
/// # See Also
///
/// * [`Color`] - Enum defining available colors.
/// * [`set_background_color`] - Function to set the background color.
/// * [`reset_color`] - Function to reset both foreground and background colors to default.
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
pub fn set_foreground_color(color: Color) {
    print(&format!("\x1B[3{}m", color as u8));
}


/// Sets the background color for subsequent text output in the terminal.
///
/// This function changes the background color of the area where text will be printed
/// after it's called. The color remains in effect until it is changed again or reset.
///
/// # Arguments
///
/// * `color` - A value of the `Color` enum representing the desired background color.
///
/// # Behavior
///
/// 1. Generates the appropriate ANSI escape sequence for the specified background color.
/// 2. Writes this sequence to stdout using the `print` function.
/// 3. All text output after this function call will use the new background color.
///
/// # Error Handling
///
/// This function does not return a Result. Any I/O errors that occur during the
/// write operation are handled by the current error handler (see `set_error_handler`).
/// By default, errors are logged to stderr.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use rpian_terminal::{set_background_color, println, Color, reset_color};
///
/// set_background_color(Color::Yellow);
/// println("This text will have a yellow background.");
/// set_background_color(Color::Green);
/// println("This text will have a green background.");
/// reset_color();
/// println("This text will have the default background.");
/// ```
///
/// Combining with foreground color:
///
/// ```
/// use rpian_terminal::{set_foreground_color, set_background_color, println, Color, reset_color};
///
/// set_foreground_color(Color::Black);
/// set_background_color(Color::White);
/// println("Black text on white background.");
/// reset_color();
/// ```
///
/// # Note
///
/// - The actual appearance of background colors may vary depending on the terminal emulator being used.
/// - Some terminals may not support all background colors or may display them differently.
/// - It's a good practice to call `reset_color()` after you're done with colored output
///   to return to the default terminal colors.
/// - This function does not affect previously output text; it only changes the background color
///   for subsequent output.
/// - In some terminal emulators, setting a background color might fill the entire width of the
///   terminal line, even if your text doesn't occupy the full width.
///
/// # See Also
///
/// * [`Color`] - Enum defining available colors.
/// * [`set_foreground_color`] - Function to set the text color.
/// * [`reset_color`] - Function to reset both foreground and background colors to default.
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
pub fn set_background_color(color: Color) {
    print(&format!("\x1B[4{}m", color as u8));
}

/// Resets both the foreground and background colors to their default values.
///
/// This function restores the terminal's default text and background colors,
/// effectively clearing any color settings previously applied using
/// `set_foreground_color` or `set_background_color`.
///
/// # Behavior
///
/// 1. Generates the ANSI escape sequence for resetting all text attributes.
/// 2. Writes this sequence to stdout using the `print` function.
/// 3. All subsequent text output will use the terminal's default colors.
///
/// # Error Handling
///
/// This function does not return a Result. Any I/O errors that occur during the
/// write operation are handled by the current error handler (see `set_error_handler`).
/// By default, errors are logged to stderr.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use rpian_terminal::{set_foreground_color, set_background_color, println, Color, reset_color};
///
/// set_foreground_color(Color::Red);
/// set_background_color(Color::Yellow);
/// println("This text will be red on a yellow background.");
/// reset_color();
/// println("This text will use the default colors.");
/// ```
///
/// Resetting between different color settings:
///
/// ```
/// use rpian_terminal::{set_foreground_color, println, Color, reset_color};
///
/// set_foreground_color(Color::Blue);
/// println("This text will be blue.");
/// reset_color();
/// println("This text will use the default color.");
/// set_foreground_color(Color::Green);
/// println("This text will be green.");
/// reset_color();
/// ```
///
/// # Note
///
/// - It's good practice to call `reset_color()` after you're done with colored output
///   to ensure that the terminal returns to its default state.
/// - This function affects both foreground and background colors simultaneously.
/// - The actual default colors may vary depending on the user's terminal settings.
/// - Calling this function does not affect previously output text; it only changes
///   the color settings for subsequent output.
///
/// # See Also
///
/// * [`set_foreground_color`] - Function to set the text color.
/// * [`set_background_color`] - Function to set the background color.
/// * [`Color`] - Enum defining available colors.
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
pub fn reset_color() {
    print("\x1B[0m");
}


/// Moves the cursor to the specified position in the terminal.
///
/// This function repositions the cursor to the given coordinates, where subsequent
/// text output will begin. The coordinate system is 1-based, with (1, 1) being the
/// top-left corner of the terminal.
///
/// # Arguments
///
/// * `x` - The column position (1-based index)
/// * `y` - The row position (1-based index)
///
/// # Behavior
///
/// 1. Checks if the requested position is within the current viewport.
/// 2. If within bounds, generates the ANSI escape sequence for cursor movement.
/// 3. Writes this sequence to stdout using the `print` function.
/// 4. If out of bounds, calls the error handler with a boundary error message.
///
/// # Error Handling
///
/// This function does not return a Result. Two types of errors may occur:
/// - Boundary errors: If the requested position is outside the viewport, a boundary
///   error is passed to the current error handler.
/// - I/O errors: Any I/O errors during writing are handled by the current error handler.
/// By default, errors are logged to stderr.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use rpian_terminal::{move_cursor_to, print, println};
///
/// // Move to the 5th column of the 3rd row and print
/// move_cursor_to(5, 3);
/// print("Hello");
///
/// // Move to the next line and print
/// move_cursor_to(1, 4);
/// println("World");
/// ```
///
/// Creating a simple box:
///
/// ```
/// use rpian_terminal::{move_cursor_to, print};
///
/// // Draw top line
/// move_cursor_to(1, 1);
/// print("+---+");
///
/// // Draw sides
/// move_cursor_to(1, 2);
/// print("|   |");
/// move_cursor_to(1, 3);
/// print("|   |");
///
/// // Draw bottom line
/// move_cursor_to(1, 4);
/// print("+---+");
/// ```
///
/// # Note
///
/// - The cursor position is relative to the current viewport, not necessarily
///   the entire terminal window.
/// - Some terminals may have limited support for cursor positioning.
/// - Moving the cursor does not clear or modify existing content in the terminal.
/// - For best results, combine this function with clear screen operations when
///   creating full-screen terminal user interfaces.
///
/// # See Also
///
/// * [`print`] - Function used for output after cursor movement.
/// * [`clear_screen`] - Function to clear the entire screen and reset cursor position.
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
pub fn move_cursor_to(x: u16, y: u16) {
    let (viewport_width, viewport_height) = get_viewport();
    if x > viewport_width || y > viewport_height {
        handle_boundary_error("Cursor position is outside viewport");
        return;
    }
    print(&format!("\x1B[{};{}H", y, x));
}

/// Clears the entire screen and moves the cursor to the top-left corner.
///
/// This function erases all content visible in the terminal window and
/// repositions the cursor to the coordinates (1, 1), which is the top-left
/// corner of the screen.
///
/// # Behavior
///
/// 1. Generates the ANSI escape sequence for clearing the entire screen.
/// 2. Writes this sequence to stdout using the `print` function.
/// 3. Calls `move_cursor_to(1, 1)` to reset the cursor position.
///
/// # Error Handling
///
/// This function does not return a Result. Any I/O errors that occur during
/// the write operation or cursor movement are handled by the current error
/// handler (see `set_error_handler`). By default, errors are logged to stderr.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use rpian_terminal::{clear_screen, println};
///
/// // Clear the screen and print a message
/// clear_screen();
/// println("The screen is now clear, and this is at the top.");
/// ```
///
/// Clearing between different outputs:
///
/// ```
/// use rpian_terminal::{clear_screen, println, wait_for_seconds};
///
/// println("This is some initial output.");
/// wait_for_seconds(2);
/// clear_screen();
/// println("The previous output is gone, and the screen is fresh.");
/// ```
///
/// # Note
///
/// - This function may behave differently on various terminal emulators.
///   Most modern terminals support this operation, but some might have
///   limited or no support.
/// - Clearing the screen does not affect the terminal's scrollback buffer.
///   Users may still be able to scroll up to see previous output, depending
///   on their terminal settings.
/// - If you only need to clear part of the screen, consider using more
///   specific functions like `clear_line()` or `clear_to_screen_end()`.
/// - After calling this function, it's a good practice to redraw any
///   persistent UI elements your application might have.
///
/// # See Also
///
/// * [`move_cursor_to`] - Function used to reposition the cursor after clearing.
/// * [`clear_line`] - Function to clear the current line.
/// * [`clear_to_screen_end`] - Function to clear from the cursor to the end of the screen.
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
pub fn clear_screen() {
    print("\x1B[2J");
    move_cursor_to(1, 1);
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

/// Reads a line from the standard input, trimming whitespace.
///
/// This function reads a line of text from the user, removes any leading or trailing
/// whitespace, and returns the result as a String. If an error occurs during reading,
/// it returns an empty string and handles the error using the current error handler.
///
/// # Return Value
///
/// Returns a `String` containing the user's input with whitespace trimmed.
/// If an error occurs during reading, an empty string is returned.
///
/// # Behavior
///
/// 1. Attempts to read a line from the standard input.
/// 2. If successful, trims whitespace from the input and returns it.
/// 3. If an error occurs, it's passed to the current error handler and an empty string is returned.
///
/// # Error Handling
///
/// This function does not return a Result. Any I/O errors that occur during the
/// read operation are handled by the current error handler (see `set_error_handler`).
/// By default, errors are logged to stderr.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use rpian_terminal::{read_line, println};
///
/// println("Please enter your name:");
/// let name = read_line();
/// println(&format!("Hello, {}!", name));
/// ```
///
/// Handling potential empty input:
///
/// ```
/// use rpian_terminal::{read_line, println};
///
/// println("Enter a command (or press Enter to skip):");
/// let command = read_line();
/// if !command.is_empty() {
///     println(&format!("Executing command: {}", command));
/// } else {
///     println("No command entered, skipping.");
/// }
/// ```
///
/// # Note
///
/// - This function will block until a line is read from the input or an error occurs.
/// - An empty string is returned both for empty input and in case of errors, so if
///   distinguishing these cases is important, additional error checking may be necessary.
///
/// # See Also
///
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
pub fn read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(e) => {
            // Handle the error using the current error handler
            handle_io_error(e);
            String::new() // Return an empty string in case of error
        }
    }
}


/// Saves the current cursor location.
///
/// This function stores the current cursor position, which can be later
/// restored using the `restore_cursor_location` function.
///
/// # Behavior
///
/// 1. Generates the ANSI escape sequence for saving the cursor position.
/// 2. Writes this sequence to stdout using the `print` function.
///
/// # Error Handling
///
/// This function does not return a Result. Any I/O errors that occur during the
/// write operation are handled by the current error handler (see `set_error_handler`).
/// By default, errors are logged to stderr.
///
/// # Examples
///
/// ```
/// use rpian_terminal::{save_cursor_location, restore_cursor_location, print, move_cursor_to};
///
/// save_cursor_location();
/// move_cursor_to(10, 5);
/// print("Temporary text");
/// restore_cursor_location();
/// ```
///
/// # Note
///
/// - Some terminals may have limited support for saving and restoring cursor positions.
/// - The saved position is typically cleared when the terminal is reset or when the
///   program exits.
///
/// # See Also
///
/// * [`restore_cursor_location`] - Function to restore the saved cursor location.
/// * [`move_cursor_to`] - Function to move the cursor to a specific position.
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
pub fn save_cursor_location() {
    print("\x1B[s");
}

/// Restores the cursor to the previously saved location.
///
/// This function moves the cursor back to the position that was last saved
/// using the `save_cursor_location` function.
///
/// # Behavior
///
/// 1. Generates the ANSI escape sequence for restoring the cursor position.
/// 2. Writes this sequence to stdout using the `print` function.
///
/// # Error Handling
///
/// This function does not return a Result. Any I/O errors that occur during the
/// write operation are handled by the current error handler (see `set_error_handler`).
/// By default, errors are logged to stderr.
///
/// # Examples
///
/// ```
/// use rpian_terminal::{save_cursor_location, restore_cursor_location, print, move_cursor_to};
///
/// save_cursor_location();
/// move_cursor_to(10, 5);
/// print("Temporary text");
/// restore_cursor_location();
/// print("Back to original position");
/// ```
///
/// # Note
///
/// - This function will have no effect if `save_cursor_location` hasn't been called previously.
/// - Some terminals may have limited support for saving and restoring cursor positions.
///
/// # See Also
///
/// * [`save_cursor_location`] - Function to save the current cursor location.
/// * [`move_cursor_to`] - Function to move the cursor to a specific position.
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
pub fn restore_cursor_location() {
    print("\x1B[u");
}

/// Makes the cursor visible.
///
/// This function sends a command to the terminal to make the cursor visible.
///
/// # Behavior
///
/// 1. Generates the ANSI escape sequence for showing the cursor.
/// 2. Writes this sequence to stdout using the `print` function.
///
/// # Error Handling
///
/// This function does not return a Result. Any I/O errors that occur during the
/// write operation are handled by the current error handler (see `set_error_handler`).
/// By default, errors are logged to stderr.
///
/// # Examples
///
/// ```
/// use rpian_terminal::{hide_cursor, show_cursor, print};
///
/// hide_cursor();
/// print("The cursor is hidden while printing this.");
/// show_cursor();
/// print("Now the cursor is visible again.");
/// ```
///
/// # Note
///
/// - The visibility of the cursor is often a user preference in terminal emulators.
///   This function may not override user settings in all environments.
///
/// # See Also
///
/// * [`hide_cursor`] - Function to hide the cursor.
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
pub fn show_cursor() {
    print("\x1B[?25h");
}

/// Hides the cursor.
///
/// This function sends a command to the terminal to make the cursor invisible.
///
/// # Behavior
///
/// 1. Generates the ANSI escape sequence for hiding the cursor.
/// 2. Writes this sequence to stdout using the `print` function.
///
/// # Error Handling
///
/// This function does not return a Result. Any I/O errors that occur during the
/// write operation are handled by the current error handler (see `set_error_handler`).
/// By default, errors are logged to stderr.
///
/// # Examples
///
/// ```
/// use rpian_terminal::{hide_cursor, show_cursor, print};
///
/// hide_cursor();
/// print("The cursor is hidden while printing this.");
/// show_cursor();
/// print("Now the cursor is visible again.");
/// ```
///
/// # Note
///
/// - It's good practice to ensure the cursor is made visible again before your
///   program exits, to avoid leaving the terminal in an unexpected state.
/// - The visibility of the cursor is often a user preference in terminal emulators.
///   This function may not override user settings in all environments.
///
/// # See Also
///
/// * [`show_cursor`] - Function to make the cursor visible.
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
pub fn hide_cursor() {
    print("\x1B[?25l");
}

/// Reads a key press from the standard input.
///
/// This function reads a line from the input, returns the first character,
/// and clears any remaining input in the buffer. If an error occurs during reading,
/// it returns a null character ('\0') and handles the error using the current error handler.
///
/// # Return Value
///
/// Returns a `char` containing the first character of the user's input.
/// If the input is empty or an error occurs during reading, a null character ('\0') is returned.
///
/// # Behavior
///
/// 1. Attempts to read a line from the standard input.
/// 2. If successful, returns the first character of the input.
/// 3. Clears any remaining input in the buffer.
/// 4. If an error occurs at any point, it's passed to the current error handler and '\0' is returned.
///
/// # Error Handling
///
/// This function does not return a Result. Any I/O errors that occur during the
/// read operation are handled by the current error handler (see `set_error_handler`).
/// By default, errors are logged to stderr.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use rpian_terminal::{read_key, println};
///
/// println("Press a key and then Enter:");
/// let key = read_key();
/// if key != '\0' {
///     println(&format!("You pressed: {}", key));
/// } else {
///     println("No key pressed or an error occurred.");
/// }
/// ```
///
/// Using in a loop:
///
/// ```
/// use rpian_terminal::{read_key, println};
///
/// println("Press keys (q to quit):");
/// loop {
///     let key = read_key();
///     if key == 'q' {
///         break;
///     }
///     println(&format!("You pressed: {}", key));
/// }
/// ```
///
/// # Note
///
/// - This function will block until a line is read from the input or an error occurs.
/// - A null character ('\0') is returned both for empty input and in case of errors,
///   so if distinguishing these cases is important, additional error checking may be necessary.
/// - This function reads a full line and returns only the first character. If you need
///   to read individual keystrokes without requiring Enter to be pressed, a different
///   approach using a crate like `termion` might be more appropriate.
///
/// # See Also
///
/// * [`read_line`] - Function to read a full line of input.
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
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
            '\0' // Return null character in case of error
        }
    }
}

/// Clears from the cursor to the end of the line.
///
/// This function erases all characters from the current cursor position
/// to the end of the current line.
///
/// # Behavior
///
/// 1. Generates the ANSI escape sequence for clearing to the end of the line.
/// 2. Writes this sequence to stdout using the `print` function.
///
/// # Error Handling
///
/// This function does not return a Result. Any I/O errors that occur during the
/// write operation are handled by the current error handler (see `set_error_handler`).
/// By default, errors are logged to stderr.
///
/// # Examples
///
/// ```
/// use rpian_terminal::{print, clear_to_line_end};
///
/// print("This is a long line of text");
/// // Move cursor back to middle of the line
/// print("\x1b[10D");
/// clear_to_line_end();
/// print(" short line");
/// // Result: "This is a  short line"
/// ```
///
/// # See Also
///
/// * [`clear_to_line_start`] - Function to clear from cursor to start of line.
/// * [`clear_line`] - Function to clear the entire line.
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
pub fn clear_to_line_end() {
    print("\x1b[K");
}

/// Clears from the cursor to the start of the line.
///
/// This function erases all characters from the current cursor position
/// to the beginning of the current line.
///
/// # Behavior
///
/// 1. Generates the ANSI escape sequence for clearing to the start of the line.
/// 2. Writes this sequence to stdout using the `print` function.
///
/// # Error Handling
///
/// This function does not return a Result. Any I/O errors that occur during the
/// write operation are handled by the current error handler (see `set_error_handler`).
/// By default, errors are logged to stderr.
///
/// # Examples
///
/// ```
/// use rpian_terminal::{print, clear_to_line_start};
///
/// print("This is a line of text");
/// // Move cursor back to middle of the line
/// print("\x1b[10D");
/// clear_to_line_start();
/// print("short ");
/// // Result: "          short text"
/// ```
///
/// # See Also
///
/// * [`clear_to_line_end`] - Function to clear from cursor to end of line.
/// * [`clear_line`] - Function to clear the entire line.
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
pub fn clear_to_line_start() {
    print("\x1b[1K");
}

/// Clears the entire line.
///
/// This function erases all characters on the current line, regardless
/// of the cursor position.
///
/// # Behavior
///
/// 1. Generates the ANSI escape sequence for clearing the entire line.
/// 2. Writes this sequence to stdout using the `print` function.
///
/// # Error Handling
///
/// This function does not return a Result. Any I/O errors that occur during the
/// write operation are handled by the current error handler (see `set_error_handler`).
/// By default, errors are logged to stderr.
///
/// # Examples
///
/// ```
/// use rpian_terminal::{println, clear_line};
///
/// println("This is a line of text");
/// clear_line();
/// println("This is on a new, clean line");
/// ```
///
/// # See Also
///
/// * [`clear_to_line_end`] - Function to clear from cursor to end of line.
/// * [`clear_to_line_start`] - Function to clear from cursor to start of line.
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
pub fn clear_line() {
    print("\x1b[2K");
}

/// Clears from the cursor to the start of the screen.
///
/// This function erases all characters from the current cursor position
/// to the beginning of the screen.
///
/// # Behavior
///
/// 1. Generates the ANSI escape sequence for clearing to the start of the screen.
/// 2. Writes this sequence to stdout using the `print` function.
///
/// # Error Handling
///
/// This function does not return a Result. Any I/O errors that occur during the
/// write operation are handled by the current error handler (see `set_error_handler`).
/// By default, errors are logged to stderr.
///
/// # Examples
///
/// ```
/// use rpian_terminal::{println, clear_to_screen_start, move_cursor_to};
///
/// println("Line 1");
/// println("Line 2");
/// println("Line 3");
/// move_cursor_to(1, 3);
/// clear_to_screen_start();
/// // Result: The first two lines are cleared, cursor is at the start of line 3
/// ```
///
/// # See Also
///
/// * [`clear_to_screen_end`] - Function to clear from cursor to end of screen.
/// * [`clear_screen`] - Function to clear the entire screen.
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
pub fn clear_to_screen_start() {
    print("\x1b[1J");
}

/// Clears from the cursor to the end of the screen.
///
/// This function erases all characters from the current cursor position
/// to the end of the screen.
///
/// # Behavior
///
/// 1. Generates the ANSI escape sequence for clearing to the end of the screen.
/// 2. Writes this sequence to stdout using the `print` function.
///
/// # Error Handling
///
/// This function does not return a Result. Any I/O errors that occur during the
/// write operation are handled by the current error handler (see `set_error_handler`).
/// By default, errors are logged to stderr.
///
/// # Examples
///
/// ```
/// use rpian_terminal::{println, clear_to_screen_end, move_cursor_to};
///
/// println("Line 1");
/// println("Line 2");
/// println("Line 3");
/// move_cursor_to(1, 2);
/// clear_to_screen_end();
/// // Result: Line 1 remains, everything from Line 2 onwards is cleared
/// ```
///
/// # See Also
///
/// * [`clear_to_screen_start`] - Function to clear from cursor to start of screen.
/// * [`clear_screen`] - Function to clear the entire screen.
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
pub fn clear_to_screen_end() {
    print("\x1b[J");
}

/// Sets the specified text attribute for subsequent text output.
///
/// This function applies a text attribute (such as bold, underline, etc.) to all
/// text that will be printed after it's called. The attribute remains in effect
/// until it is changed or reset.
///
/// # Arguments
///
/// * `attribute` - A value of the `Attribute` enum representing the desired text attribute.
///
/// # Behavior
///
/// 1. Generates the appropriate ANSI escape sequence for the specified attribute.
/// 2. Writes this sequence to stdout using the `print` function.
/// 3. All text output after this function call will use the new attribute.
///
/// # Error Handling
///
/// This function does not return a Result. Any I/O errors that occur during the
/// write operation are handled by the current error handler (see `set_error_handler`).
/// By default, errors are logged to stderr.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use rpian_terminal::{set_attribute, println, Attribute, reset_attributes};
///
/// set_attribute(Attribute::Bold);
/// println("This text will be bold.");
/// set_attribute(Attribute::Underscore);
/// println("This text will be underscored.");
/// reset_attributes();
/// println("This text will have default attributes.");
/// ```
///
/// # Note
///
/// - The actual appearance of text attributes may vary depending on the terminal emulator being used.
/// - Some terminals may not support all attributes or may display them differently.
/// - It's a good practice to call `reset_attributes()` after you're done with styled output
///   to return to the default text style.
///
/// # See Also
///
/// * [`Attribute`] - Enum defining available text attributes.
/// * [`reset_attributes`] - Function to reset all text attributes to default.
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
pub fn set_attribute(attribute: Attribute) {
    print(&format!("\x1B[{}m", attribute as u8));
}

/// Resets all text attributes to their default values.
///
/// This function restores the terminal's default text attributes,
/// effectively clearing any attribute settings previously applied using `set_attribute`.
///
/// # Behavior
///
/// 1. Calls `set_attribute` with `Attribute::Reset`.
/// 2. All subsequent text output will use the terminal's default attributes.
///
/// # Error Handling
///
/// This function does not return a Result. Any I/O errors that occur during the
/// write operation are handled by the current error handler (see `set_error_handler`).
/// By default, errors are logged to stderr.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use rpian_terminal::{set_attribute, println, Attribute, reset_attributes};
///
/// set_attribute(Attribute::Bold);
/// println("This text will be bold.");
/// reset_attributes();
/// println("This text will use default attributes.");
/// ```
///
/// # Note
///
/// - It's good practice to call `reset_attributes()` after you're done with styled output
///   to ensure that the terminal returns to its default state.
/// - This function affects all text attributes simultaneously.
/// - The actual default attributes may vary depending on the user's terminal settings.
///
/// # See Also
///
/// * [`set_attribute`] - Function to set a specific text attribute.
/// * [`Attribute`] - Enum defining available text attributes.
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
pub fn reset_attributes() {
    set_attribute(Attribute::Reset);
}

/// Sets the viewport size.
///
/// # Arguments
///
/// * `width` - The width of the viewport.
/// * `height` - The height of the viewport.
pub fn set_viewport(width: u16, height: u16) {
    VIEWPORT_WIDTH.store(width, Ordering::Relaxed);
    VIEWPORT_HEIGHT.store(height, Ordering::Relaxed);
}

/// Gets the current viewport size.
///
/// # Returns
///
/// Returns a tuple containing the width and height of the viewport.
pub fn get_viewport() -> (u16, u16) {
    (
        VIEWPORT_WIDTH.load(Ordering::Relaxed),
        VIEWPORT_HEIGHT.load(Ordering::Relaxed),
    )
}

/// Draws a diagonal line in the terminal using the specified symbol.
///
/// This function uses Bresenham's line algorithm to draw a line between two points
/// in the terminal. The line is drawn using the provided symbol character.
///
/// # Arguments
///
/// * `symbol` - The character to use for drawing the line
/// * `x1` - The x-coordinate of the starting point (1-based index)
/// * `y1` - The y-coordinate of the starting point (1-based index)
/// * `x2` - The x-coordinate of the ending point (1-based index)
/// * `y2` - The y-coordinate of the ending point (1-based index)
///
/// # Behavior
///
/// 1. Checks if the line's start and end points are within the current viewport.
/// 2. If within bounds, uses Bresenham's algorithm to determine which points to draw.
/// 3. For each point in the line:
///    - Moves the cursor to the point's position
///    - Prints the specified symbol
/// 4. If out of bounds, calls the error handler with a boundary error message.
///
/// # Error Handling
///
/// This function does not return a Result. Two types of errors may occur:
/// - Boundary errors: If any part of the line is outside the viewport, a boundary
///   error is passed to the current error handler.
/// - I/O errors: Any I/O errors during writing are handled by the current error handler.
/// By default, errors are logged to stderr.
///
/// # Examples
///
/// Drawing a simple diagonal line:
///
/// ```
/// use rpian_terminal::{diagonal_line, clear_screen};
///
/// clear_screen();
/// diagonal_line('*', 1, 1, 10, 5);
/// ```
///
/// Creating a triangle:
///
/// ```
/// use rpian_terminal::{diagonal_line, clear_screen};
///
/// clear_screen();
/// diagonal_line('/', 1, 5, 5, 1);   // Left side
/// diagonal_line('\\', 5, 1, 9, 5);  // Right side
/// diagonal_line('-', 1, 5, 9, 5);   // Base
/// ```
///
/// # Note
///
/// - The function uses a 1-based coordinate system, where (1, 1) is the top-left corner.
/// - The line is drawn using individual characters, which may result in a 'stepped'
///   appearance for diagonal lines, especially on terminals with non-square character cells.
/// - This function does not clear the screen before drawing. It's often a good idea
///   to call `clear_screen()` before starting a new drawing.
/// - The function will attempt to draw as much of the line as possible within the
///   viewport, even if part of it would be out of bounds.
/// - For best visual results, consider using a symmetric character as the symbol
///   (e.g., '*', '+', '·').
///
/// # See Also
///
/// * [`move_cursor_to`] - Function used internally for positioning the cursor.
/// * [`put_char`] - Function used internally for outputting the symbol.
/// * [`clear_screen`] - Function to clear the screen before drawing.
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
pub fn diagonal_line(symbol: char, x1: u16, y1: u16, x2: u16, y2: u16) {
    let (viewport_width, viewport_height) = get_viewport();

    if x1 > viewport_width || y1 > viewport_height || x2 > viewport_width || y2 > viewport_height {
        handle_boundary_error("Line coordinates are outside viewport");
        return;
    }

    // Bresenham's line algorithm (keep the existing implementation)
    let dx = (x2 as i32 - x1 as i32).abs();
    let dy = -(y2 as i32 - y1 as i32).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut x = x1 as i32;
    let mut y = y1 as i32;

    loop {
        if x >= 0 && x < viewport_width as i32 && y >= 0 && y < viewport_height as i32 {
            move_cursor_to(x as u16, y as u16);
            put_char(symbol);
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
}





/// Writes the given text to the standard output and flushes the buffer.
///
/// This function is the core output mechanism for the rpian-terminal library.
/// It writes the provided text to stdout and immediately flushes the buffer
/// to ensure the output is displayed. Any I/O errors that occur during this
/// process are handled internally using the current error handler.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to be printed.
///
/// # Behavior
///
/// 1. Attempts to write the entire text to stdout.
/// 2. If successful, attempts to flush stdout.
/// 3. If either operation fails, the error is passed to the current error handler.
///
/// # Error Handling
///
/// This function does not return a Result. Instead, any I/O errors are passed
/// to the `handle_io_error` function, which uses the currently set error handler
/// (see `set_error_handler`). By default, errors are logged to stderr.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use rpian_terminal::print;
///
/// print("Hello, world!");
/// ```
///
/// Using with custom error handling:
///
/// ```
/// use rpian_terminal::{print, ErrorHandler, set_error_handler};
/// use std::io;
///
/// struct MyErrorHandler;
///
/// impl ErrorHandler for MyErrorHandler {
///     fn handle_io_error(&self, error: io::Error) {
///         // Custom error handling logic
///         eprintln!("Custom I/O error handling: {}", error);
///     }
///
///     fn handle_boundary_error(&self, message: &str) {
///         // Implementation for boundary errors
///     }
/// }
///
/// fn main() {
///     unsafe {
///         set_error_handler(&MyErrorHandler);
///     }
///
///     print("This will use the custom error handler if an I/O error occurs.");
/// }
/// ```
///
/// # Note
///
/// This function is intended for internal use within the rpian-terminal library.
/// Library users should generally use the public API functions, which internally
/// use `print` for output operations.
pub fn print(text: &str) {
    if let Err(e) = io::stdout().write_all(text.as_bytes()).and_then(|_| io::stdout().flush()) {
        handle_io_error(e);
    }
}

/// Writes the given text to the standard output, followed by a newline, and flushes the buffer.
///
/// This function is a convenience wrapper around the `print` function. It appends a newline
/// character (`\n`) to the provided text before calling `print`, ensuring that subsequent
/// output will start on a new line.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to be printed.
///
/// # Behavior
///
/// 1. Appends a newline character to the provided text.
/// 2. Calls the `print` function with the modified text.
///
/// # Error Handling
///
/// Like `print`, this function does not return a Result. Any I/O errors that occur
/// during the write or flush operations are handled by the current error handler
/// (see `set_error_handler`). By default, errors are logged to stderr.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use rpian_terminal::println;
///
/// println("Hello, world!");
/// println("This will be on a new line.");
/// ```
///
/// Comparing `print` and `println`:
///
/// ```
/// use rpian_terminal::{print, println};
///
/// print("This will not end with a newline. ");
/// print("This will be on the same line.");
/// println("This will end the line.");
/// println("This will be on a new line.");
/// ```
///
/// # Note
///
/// While this function is primarily used internally by the rpian-terminal library,
/// it is also available for direct use when you need to ensure output ends with a newline.
/// For most terminal manipulation tasks, prefer the specific functions provided by the library.
///
/// # See Also
///
/// * [`print`] - The underlying function used for output without an automatic newline.
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
pub fn println(text: &str) {
    print(&(text.to_owned() + "\n"));
}

/// Writes a single character to the standard output and flushes the buffer.
///
/// This function is designed for efficient single-character output in the rpian-terminal library.
/// It converts the provided character to a string and then uses the `print` function to output it.
/// This approach ensures consistent error handling and buffer flushing across all output functions.
///
/// # Arguments
///
/// * `ch` - The character to be printed.
///
/// # Behavior
///
/// 1. Converts the input character to a string.
/// 2. Calls the `print` function with this single-character string.
///
/// # Error Handling
///
/// Like other output functions in this library, `put_char` does not return a Result.
/// Any I/O errors that occur during the write or flush operations are handled by
/// the current error handler (see `set_error_handler`). By default, errors are logged to stderr.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use rpian_terminal::put_char;
///
/// put_char('A');
/// put_char('B');
/// put_char('\n'); // Note: This will start a new line
/// put_char('C');
/// ```
///
/// Using in a loop:
///
/// ```
/// use rpian_terminal::put_char;
///
/// for c in "Hello, world!".chars() {
///     put_char(c);
/// }
/// put_char('\n');
/// ```
///
/// # Note
///
/// While this function can be used directly, it's often more efficient to use `print` or `println`
/// for outputting strings or multiple characters. `put_char` is particularly useful in scenarios
/// where character-by-character output is necessary, such as in certain drawing or animation functions.
///
/// # See Also
///
/// * [`print`] - The underlying function used for string output.
/// * [`println`] - Function for printing a line of text with a trailing newline.
/// * [`ErrorHandler`] - The trait used for custom error handling.
/// * [`set_error_handler`] - Function to set a custom error handler.
pub fn put_char(ch: char) {
    print(&ch.to_string());
}

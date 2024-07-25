use std::io::{self, Write};

/// Moves the cursor to the specified (x, y) coordinates in the terminal.
pub fn teleport_to(x: u32, y: u32) {
    print!("\x1B[{};{}H", y, x);
    io::stdout().flush().unwrap();
}

/// Prints a single character to the terminal and immediately flushes the output.
pub fn put_char(ch: char) {
    print!("{}", ch);
    io::stdout().flush().unwrap();
}

/// Clears the entire screen and moves the cursor to position (1, 1).
pub fn clear_screen() {
    print!("\x1B[2J");
    teleport_to(1, 1);
    io::stdout().flush().unwrap();
}
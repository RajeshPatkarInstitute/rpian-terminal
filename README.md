# rpian-terminal

`rpian-terminal` is a lightweight Rust crate that provides simple functions for basic terminal manipulation and color output. It allows you to move the cursor, clear the screen, print characters, and set text colors with ease, all while handling potential I/O errors.

## Features

- Move the cursor to specific coordinates in the terminal
- Clear the entire terminal screen
- Print individual characters with immediate flushing
- Set foreground and background colors for text output
- Reset text formatting to default
- Consistent error handling across all functions

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rpian-terminal = "0.2.0"
```

## Usage

Here's a quick example of how to use `rpian-terminal`:

```rust
use rpian_terminal::{clear_screen, teleport_to, put_char, Color, set_foreground_color, set_background_color, reset_color};
use std::io;

fn main() -> io::Result<()> {
    // Clear the screen
    clear_screen()?;

    // Set foreground color to red and print a character
    set_foreground_color(Color::Red)?;
    teleport_to(10, 5)?;
    put_char('H')?;

    // Set background color to blue and print another character
    set_background_color(Color::Blue)?;
    teleport_to(11, 5)?;
    put_char('i')?;

    // Reset colors
    reset_color()?;

    // Move cursor and print normal text
    teleport_to(1, 10)?;
    println!("Colors have been reset.");

    Ok(())
}
```

## API Reference

### Terminal Manipulation

#### `teleport_to(x: u32, y: u32) -> io::Result<()>`

Moves the cursor to the specified (x, y) coordinates in the terminal.

#### `put_char(ch: char) -> io::Result<()>`

Prints a single character to the terminal and immediately flushes the output.

#### `clear_screen() -> io::Result<()>`

Clears the entire screen and moves the cursor to position (1, 1).

### Color Manipulation

#### `set_foreground_color(color: Color) -> io::Result<()>`

Sets the foreground color for subsequent text output.

#### `set_background_color(color: Color) -> io::Result<()>`

Sets the background color for subsequent text output.

#### `reset_color() -> io::Result<()>`

Resets all text formatting to default.

### Color Enum

```rust
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
```

Use this enum to specify colors for `set_foreground_color` and `set_background_color` functions.

## Error Handling

All functions in this crate return `io::Result<()>`, allowing you to handle potential I/O errors. This means you can use these functions in a `Result`-aware context and handle or propagate errors as needed.

Example of error handling:

```rust
fn write_colored_text() -> io::Result<()> {
    set_foreground_color(Color::Green)?;
    put_char('H')?;
    put_char('e')?;
    put_char('l')?;
    put_char('l')?;
    put_char('o')?;
    reset_color()?;
    Ok(())
}
```

## Terminal Compatibility

`rpian-terminal` uses ANSI escape codes for terminal manipulation and color output. It should work on most Unix-like systems and modern Windows terminals that support ANSI escape sequences.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome from students enrolled in the Rust Programming Course @ Rajesh Patkar Institute Of Software Engineering. Here's how you can contribute:

1. Fork the repository
2. Create a new branch for your feature or bug fix
3. Make your changes and commit them with a clear commit message
4. Push your changes to your fork
5. Submit a pull request to the main repository

Please ensure your code adheres to the existing style and includes appropriate tests.

## Reporting Issues

If you encounter any bugs or have feature requests, please open an issue on the GitHub repository. Provide as much detail as possible, including steps to reproduce the issue and your environment details.

## Credits

Created by Rajesh Patkar
# rpian-terminal

A simple Rust library for terminal manipulation and input/output operations, designed for educational purposes.

## IMPORTANT NOTICE

**This library is designed specifically for exploratory exercises in the Rust course at Rajesh Patkar Institute of Software Engineering. It is NOT intended for use in production environments. The primary goal is to facilitate learning and understanding of Rust concepts related to terminal interactions. Students are encouraged to identify limitations and problems in the design and features of the library and suggest solutions.**

## Features

- Color manipulation (foreground and background) using the `Color` enum
- Text attribute manipulation using the `Attribute` enum
- Cursor movement and screen clearing
- Character and line input
- Timed waits (seconds, milliseconds, microseconds)
- Cursor visibility control
- Basic error handling using `io::Result`
- Various screen and line clearing functions

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rpian-terminal = "0.4.0"
```

## Usage

Here's a quick example of how to use some of the functions:

```rust
use rpian_terminal::*;

fn main() -> std::io::Result<()> {
    clear_screen()?;
    set_foreground_color(Color::Green)?;
    set_attribute(Attribute::Bright)?;
    println("Welcome to rpian-terminal!")?;
    reset_color()?;
    reset_attributes()?;

    move_cursor_to(1, 3)?;
    print("Enter your name: ")?;
    let name = read_line()?;

    move_cursor_to(1, 5)?;
    set_foreground_color(Color::Blue)?;
    set_attribute(Attribute::Underscore)?;
    println(&format!("Hello, {}!", name))?;

    wait_for_seconds(2); // Wait for 2 seconds

    clear_screen()?;
    Ok(())
}
```

## API Overview

- Color enum: `Color` (Black, Red, Green, Yellow, Blue, Magenta, Cyan, White)
- Attribute enum: `Attribute` (Reset, Bright, Dim, Underscore, Blink, Reverse, Hidden)
- Color manipulation: `set_foreground_color`, `set_background_color`, `reset_color`
- Attribute manipulation: `set_attribute`, `reset_attributes`
- Cursor control: `move_cursor_to`, `save_cursor_location`, `restore_cursor_location`, `show_cursor`, `hide_cursor`
- Screen manipulation: `clear_screen`, `put_char`, `clear_to_line_end`, `clear_to_line_start`, `clear_line`, `clear_to_screen_start`, `clear_to_screen_end`
- Input functions: `read_char`, `read_line`, `read_key`
- Output functions: `print`, `println`
- Wait functions: `wait_for_seconds`, `wait_for_millis`, `wait_for_micros`

Most functions return `io::Result<()>` for consistent error handling.

For detailed API documentation, please run `cargo doc --open` in your project directory.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

As this is an educational project, contributions are limited to students currently participating in the Rust course at Rajesh Patkar Institute of Software Engineering. This restriction ensures that the library remains aligned with the course curriculum and learning objectives.

Students are encouraged to:
- Identify and report limitations or issues in the library's design and features
- Propose and implement solutions or improvements
- Experiment with the code and share learnings with classmates

## Disclaimer

This library is not optimized for performance or comprehensive error handling. It is intentionally kept simple for educational purposes and uses basic ANSI escape sequences for terminal control. For real-world applications, consider using more robust and feature-rich crates like `termion`, `crossterm`, or `ncurses`.

## Changelog

### 0.4.0
- Added `Attribute` enum for text attribute manipulation
- Added `set_attribute` and `reset_attributes` functions
- Added new screen and line clearing functions: `clear_to_line_end`, `clear_to_line_start`, `clear_line`, `clear_to_screen_start`, `clear_to_screen_end`

### 0.3.0
- Initial public release
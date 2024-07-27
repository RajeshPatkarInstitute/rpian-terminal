# rpian-terminal

A simple Rust library for terminal manipulation and input/output operations, designed for educational purposes.

## IMPORTANT NOTICE

**This library is designed specifically for exploratory exercises in the Rust course at Rajesh Patkar Institute of Software Engineering. It is NOT intended for use in production environments. The primary goal is to facilitate learning and understanding of Rust concepts related to terminal interactions. Students are encouraged to identify limitations and problems in the design and features of the library and suggest solutions.**

## Features

- Color manipulation (foreground and background)
- Text attribute manipulation (bold, underline, etc.)
- Cursor movement and screen clearing
- Character and line input
- Timed waits (seconds, milliseconds, microseconds)
- Cursor visibility control
- Various screen and line clearing functions
- Drawing primitives (lines, boxes)
- Viewport management

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rpian-terminal = "0.5.1"
```

## Usage

Here's a quick example of how to use some of the functions:

```rust
use rpian_terminal::*;

fn main() -> std::io::Result<()> {
    set_viewport(80, 24);
    clear_screen()?;
    
    set_foreground_color(Color::Green)?;
    set_attribute(Attribute::Bright)?;
    println("Welcome to rpian-terminal!")?;
    reset_color()?;
    reset_attributes()?;

    draw_box(5, 3, 70, 18, BoxStyle::Double)?;
    draw_box(10, 5, 60, 14, BoxStyle::Single)?;

    move_cursor_to(15, 10)?;
    print("Enter your name: ")?;
    let name = read_line()?;

    move_cursor_to(15, 12)?;
    set_foreground_color(Color::Blue)?;
    set_attribute(Attribute::Underscore)?;
    println(&format!("Hello, {}!", name))?;

    wait_for_seconds(2);
    clear_screen()?;
    Ok(())
}
```

## API Overview

### Colors and Attributes
- `Color` enum: Black, Red, Green, Yellow, Blue, Magenta, Cyan, White
- `Attribute` enum: Reset, Bright, Dim, Underscore, Blink, Reverse, Hidden
- Functions: `set_foreground_color`, `set_background_color`, `reset_color`, `set_attribute`, `reset_attributes`

### Cursor and Screen Control
- Functions: `move_cursor_to`, `clear_screen`, `save_cursor_location`, `restore_cursor_location`, `show_cursor`, `hide_cursor`

### Input and Output
- Functions: `read_key`, `read_line`, `print`, `println`

### Timing
- Functions: `wait_for_seconds`, `wait_for_millis`, `wait_for_micros`

### Screen Clearing
- Functions: `clear_to_line_end`, `clear_to_line_start`, `clear_line`, `clear_to_screen_start`, `clear_to_screen_end`

### Drawing
- `BoxStyle` enum: Single, Double
- `BoxChar` enum: Various box-drawing characters
- Functions: `horizontal_line`, `vertical_line`, `diagonal_line`, `draw_box`

### Viewport Management
- Functions: `set_viewport`, `get_viewport`

## Error Handling

Most functions return `io::Result<()>` for consistent error handling.

## Documentation

For detailed API documentation, run `cargo doc --open` in your project directory.

## Contributing

As this is an educational project, contributions are limited to students currently participating in the Rust course at Rajesh Patkar Institute of Software Engineering. This restriction ensures that the library remains aligned with the course curriculum and learning objectives.

Students are encouraged to:
- Identify and report limitations or issues in the library's design and features
- Propose and implement solutions or improvements
- Experiment with the code and share learnings with classmates

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This library is not optimized for performance or comprehensive error handling. It is intentionally kept simple for educational purposes and uses basic ANSI escape sequences for terminal control. For real-world applications, consider using more robust and feature-rich crates like `termion`, `crossterm`, or `ncurses`.

## Changelog

### 0.5.1
- Removed warnings
### 0.5.0
- Added viewport management with `set_viewport` and `get_viewport` functions
- Introduced drawing functions: `horizontal_line`, `vertical_line`, `diagonal_line`, and `draw_box`
- Added `BoxStyle` and `BoxChar` enums for box drawing
- Deprecated `read_char` in favor of `read_key`
- Added various screen and line clearing functions
- Improved documentation and examples
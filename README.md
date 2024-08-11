# rpian-terminal

A Rust library for terminal manipulation and drawing, designed for educational purposes at Rajesh Patkar Institute of Software Engineering.

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
- Drawing primitives (lines, boxes with multiple styles, shaded rectangles)
- Viewport management
- Unicode support for various symbols (arrows, stars, math symbols, chess pieces, emojis, Braille patterns)
- Global error handler for consistent error management
- Enhanced line drawing capabilities with various styles and directions

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rpian-terminal = "0.8.1"
```

## Usage

Here's a quick example of how to use some of the functions:

```rust
use rpian_terminal::*;
use rbox::{draw_box, BoxStyle};
use line::{Line, Direction, Shape};

fn main() {
    set_viewport(80, 24);
    clear_screen();
    
    set_foreground_color(Color::Green);
    set_attribute(Attribute::Bright);
    println("Welcome to rpian-terminal!");
    reset_color();
    reset_attributes();

    draw_box(5, 3, 70, 18, BoxStyle::Double);
    draw_box(10, 5, 60, 14, BoxStyle::SingleRounded);

    let mut line = Line::new();
    line.x = 15;
    line.y = 10;
    line.size = 20;
    line.direction = Direction::East;
    line.show(Some(2));

    move_cursor_to(15, 12);
    print("Enter your name: ");
    let name = read_line();

    move_cursor_to(15, 14);
    set_foreground_color(Color::Blue);
    set_attribute(Attribute::Underscore);
    println(&format!("Hello, {}!", name));

    wait_for_seconds(2);
    clear_screen();
}
```

Note: Error handling is done internally using the global error handler.

## Modules

The library is organized into several modules:

- `arrow`: Provides arrow symbols
- `braille`: Implements Braille patterns
- `chess`: Offers chess piece symbols
- `emoji`: Includes various emoji symbols
- `math`: Provides mathematical symbols
- `rbox`: Implements box drawing and shading functions
- `star`: Offers star symbols
- `triangle`: Provides triangle symbols
- `error`: Implements custom error handling
- `line`: Implements enhanced line drawing capabilities
- `circle`: Provides circle symbols

## API Overview

### Colors and Attributes
- `Color` enum: Black, Red, Green, Yellow, Blue, Magenta, Cyan, White
- `Attribute` enum: Reset, Bright, Dim, Underscore, Blink, Reverse, Hidden
- Functions: `set_foreground_color`, `set_background_color`, `reset_color`, `set_attribute`, `reset_attributes`

### Cursor and Screen Control
- Functions: `move_cursor_to`, `clear_screen`, `save_cursor_location`, `restore_cursor_location`, `show_cursor`, `hide_cursor`

### Input and Output
- Functions: 
  - `read_key`: Reads a single keypress
  - `read_line`: Reads a full line of input
  - `print`: Outputs a string
  - `println`: Outputs a string followed by a newline
  - `put_char`: Outputs a single character

### Timing
- Functions: `wait_for_seconds`, `wait_for_millis`, `wait_for_micros`

### Screen Clearing
- Functions: `clear_to_line_end`, `clear_to_line_start`, `clear_line`, `clear_to_screen_start`, `clear_to_screen_end`

### Drawing
- `BoxStyle` enum: Single, Double, SingleRounded, DoubleRounded, Dotted, Dashed
- `ShadeStyle` enum: Light, Medium, Dark, Solid
- Functions: 
  - `diagonal_line`: Draws a diagonal line
  - `draw_box`: Draws a box with specified style
  - `draw_shaded_rectangle`: Draws a shaded rectangle
  - `hide_box`: Erases a previously drawn box

### Line Drawing
- `Line` struct: Represents a line with customizable properties
- `Direction` enum: North, South, East, West, NorthEast, NorthWest, SouthEast, SouthWest
- `LineStyle` struct: Customizes line appearance
- Functions:
  - `horizontal_line`: Draws a horizontal line
  - `vertical_line`: Draws a vertical line
  - `diagonal_line`: Draws a diagonal line

### Viewport Management
- Functions: 
  - `set_viewport(width: u16, height: u16)`: Sets the viewport size
  - `get_viewport() -> (u16, u16)`: Gets the current viewport size

### Symbol Modules
Each symbol module (`arrow`, `braille`, `chess`, `emoji`, `math`, `star`, `triangle`, `circle`) provides enums and functions to access various Unicode symbols.

### Error Handling
- Custom `ErrorHandler` trait for flexible error management
- Global error handler that can be set using `set_error_handler`
- Functions: `handle_io_error`, `handle_boundary_error`

## Error Handling

The library uses a global error handler for consistent error management across all functions. Most functions no longer return `Result` types, simplifying usage. Errors are handled internally using the global error handler.

Users can implement the `ErrorHandler` trait to create custom error handling logic:

```rust
use rpian_terminal::{ErrorHandler, set_error_handler};
use std::io;

struct MyErrorHandler;

impl ErrorHandler for MyErrorHandler {
    fn handle_io_error(&self, error: io::Error) {
        eprintln!("Custom I/O error handling: {}", error);
    }

    fn handle_boundary_error(&self, message: &str) {
        eprintln!("Custom boundary error handling: {}", message);
    }
}

fn main() {
    unsafe {
        set_error_handler(&MyErrorHandler);
    }
    // Rest of your code...
}
```

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

### 0.8.0
- Enhanced line drawing capabilities with the new `line` module
- Added `Line` struct for more flexible line drawing
- Introduced `Direction` enum for specifying line directions
- Added `LineStyle` struct for customizing line appearance
- Implemented new line drawing functions: `horizontal_line`, `vertical_line`, `diagonal_line`
- Added `circle` module with circle symbols
- Updated existing modules to use the new line drawing system
- Improved documentation and examples

### 0.7.0
- Implemented a global error handler system
- Added custom `ErrorHandler` trait for flexible error management
- Removed `Result` return types from most functions for simplified usage
- Enhanced `rbox` module with new box drawing and shading functions
- Added `hide_box` function for removing drawn boxes
- Added `put_char` function for single character output
- Updated viewport functions to use `u16` for dimensions
- Improved documentation and examples
- Updated existing modules to use the new error handling system

### 0.6.0
- Reorganized codebase into modules
- Added new modules: arrow, braille, chess, emoji, math, star, triangle
- Improved box drawing with new styles and shading options
- Added Unicode symbol support for various categories
- Enhanced error handling with custom error types
- Updated documentation and examples
# rpian-terminal

`rpian-terminal` is a lightweight Rust crate that provides simple functions for basic terminal manipulation. It allows you to move the cursor, clear the screen, and print characters with ease.

## Features

- Move the cursor to specific coordinates in the terminal
- Clear the entire terminal screen
- Print individual characters with immediate flushing

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rpian-terminal = "0.1.0"
```

## Usage

Here's a quick example of how to use `rpian-terminal`:

```rust
use rpian_terminal::{clear_screen, teleport_to, putchar};

fn main() {
    // Clear the screen
    clear_screen();

    // Move cursor to position (10, 5) and print a character
    teleport_to(10, 5);
    putchar('H');

    // Move cursor to position (11, 5) and print another character
    teleport_to(11, 5);
    putchar('i');
}
```

## API Reference

### `teleport_to(x: u32, y: u32)`

Moves the cursor to the specified (x, y) coordinates in the terminal.

### `putchar(ch: char)`

Prints a single character to the terminal and immediately flushes the output.

### `clear_screen()`

Clears the entire screen and moves the cursor to position (1, 1).

## License

This project is licensed under [MIT].

## Contributing

Contributions are welcome from students enrolled in Rust Programming Course @ Rajesh Patkar Institute Of Software Engg.

## Credits

Created by Rajesh Patkar

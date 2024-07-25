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
rpian-terminal = "0.1.1"  # Update this to your new version number
```

## Usage

Here's a quick example of how to use `rpian-terminal`:

```rust
use rpian_terminal::{clear_screen, teleport_to, put_char};

fn main() {
    // Clear the screen
    clear_screen();

    // Move cursor to position (10, 5) and print a character
    teleport_to(10, 5);
    put_char('H');

    // Move cursor to position (11, 5) and print another character
    teleport_to(11, 5);
    put_char('i');
}
```

## API Reference

### `teleport_to(x: u32, y: u32)`

Moves the cursor to the specified (x, y) coordinates in the terminal.

### `put_char(ch: char)`

Prints a single character to the terminal and immediately flushes the output.

### `clear_screen()`

Clears the entire screen and moves the cursor to position (1, 1).

## Error Handling

This crate uses `std::io::Write` for terminal operations. If any I/O errors occur during these operations, they will be silently ignored. In future versions, we may add more robust error handling and reporting.

## Terminal Compatibility

`rpian-terminal` uses ANSI escape codes for terminal manipulation. It should work on most Unix-like systems and modern Windows terminals that support ANSI escape sequences.

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
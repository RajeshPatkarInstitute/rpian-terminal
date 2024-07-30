use rpian_terminal::*;
use std::io;

fn main() -> io::Result<()> {
    // Clear the screen and set up the viewport
    clear_screen()?;
    set_viewport(80, 24);

    // Draw the single box with a diamond inside
    draw_diamond(10, 10, "r1")?;

    // Wait for user input before exiting
    println("Press Enter to exit.")?;
    read_line()?;
    clear_screen()?;
    Ok(())
}

fn draw_diamond(x: u16, y: u16, text: &str) -> io::Result<()> {
    // Draw the diamond using \ and /
    move_cursor_to(x + 1, y)?;
    print("/")?;
    move_cursor_to(x + 2, y + 1)?;
    print("\\")?;
    move_cursor_to(x + 2, y + 2)?;
    print("/")?;
    move_cursor_to(x + 1, y + 3)?;
    print("\\")?;
    move_cursor_to(x + 1, y + 1)?;
    print("\\")?;
    move_cursor_to(x + 1, y + 2)?;
    print("/")?;

    // Print the text in the middle of the diamond
    move_cursor_to(x + 1, y + 1)?;
    print(text)?;

    Ok(())
}

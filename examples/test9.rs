use rpian_terminal::*;
use std::io;

fn main() -> io::Result<()> {
    // Clear the screen and set up the viewport
    clear_screen()?;
    set_viewport(80, 24);

    // Draw the diamond
    draw_detailed_diamond(10, 20, "r1")?;

    // Wait for user input before exiting
    println("Press Enter to exit.")?;
    read_line()?;
    clear_screen()?;
    Ok(())
}

fn draw_detailed_diamond(x: u16, y: u16, text: &str) -> io::Result<()> {
    // Draw the diamond using *, ╲, and ╱
    move_cursor_to(x + 2, y)?;
    print("*")?;
    move_cursor_to(x + 1, y + 1)?;
    print("╱")?;
    move_cursor_to(x + 3, y + 1)?;
    print("╲")?;
    move_cursor_to(x, y + 2)?;
    print("*")?;
    move_cursor_to(x + 4, y + 2)?;
    print("*")?;
    move_cursor_to(x + 1, y + 3)?;
    print("╲")?;
    move_cursor_to(x + 3, y + 3)?;
    print("╱")?;
    move_cursor_to(x + 2, y + 4)?;
    print("*")?;

    // Print the text in the middle of the diamond
    move_cursor_to(x + 2 - (text.len() as u16 / 2), y + 2)?;
    print(text)?;

    Ok(())
}

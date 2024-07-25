use rpian_terminal::*;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Clear the screen and set initial color
    clear_screen()?;
    set_foreground_color(Color::Green)?;
    println("Welcome to the rpian-terminal demo!")?;
    reset_color()?;

    // Demonstrate cursor movement and color changes
    move_cursor_to(1, 3)?;
    set_foreground_color(Color::Blue)?;
    println("This text is blue and at position (1, 3)")?;

    move_cursor_to(5, 5)?;
    set_background_color(Color::Yellow)?;
    set_foreground_color(Color::Black)?;
    println("Black text on yellow background at (5, 5)")?;
    reset_color()?;

    // Demonstrate input functions
    move_cursor_to(1, 7)?;
    print("Press any key to continue...")?;
    let key = read_key()?;
    move_cursor_to(1, 8)?;
    println(&format!("You pressed: {}", key))?;

    // Demonstrate cursor visibility
    move_cursor_to(1, 10)?;
    println("The cursor will now disappear for 3 seconds...")?;
    hide_cursor()?;
    wait_for_seconds(3);
    show_cursor()?;

    // Demonstrate save/restore cursor position
    move_cursor_to(1, 12)?;
    println("Saving cursor position...")?;
    save_cursor_location()?;
    
    move_cursor_to(5, 14)?;
    set_foreground_color(Color::Magenta)?;
    println("This text is in a different position and color.")?;
    reset_color()?;
    
    wait_for_seconds(2);
    restore_cursor_location()?;
    println("Cursor position restored!")?;

    // Demonstrate different wait durations
    move_cursor_to(1, 16)?;
    println("Waiting for 2 seconds...")?;
    wait_for_seconds(2);
    println("Waited for 2 seconds")?;

    println("Waiting for 1000 milliseconds...")?;
    wait_for_millis(1000);
    println("Waited for 1000 milliseconds")?;

    println("Waiting for 500000 microseconds...")?;
    wait_for_micros(500000);
    println("Waited for 500000 microseconds")?;

    // Final user interaction
    move_cursor_to(1, 22)?;
    set_foreground_color(Color::Cyan)?;
    print("Enter your name: ")?;
    reset_color()?;
    let name = read_line()?;

    move_cursor_to(1, 24)?;
    set_foreground_color(Color::Green)?;
    println(&format!("Thank you for trying rpian-terminal, {}!", name))?;
    reset_color()?;

    // Clean up
    move_cursor_to(1, 26)?;
    println("Press any key to exit...")?;
    let _ = read_char()?;
    clear_screen()?;

    Ok(())
}
use rpian_terminal::*;
use std::io;

fn main() -> io::Result<()> {
    // Clear screen and hide cursor
    clear_screen()?;
    hide_cursor()?;

    // Test colors
    println("Testing colors:")?;
    for color in &[Color::Red, Color::Green, Color::Blue, Color::Yellow, Color::Magenta, Color::Cyan, Color::White] {
        set_foreground_color(*color)?;
        print("Colored text ")?;
    }
    reset_color()?;
    println("")?;

    // Test background colors
    println("\nTesting background colors:")?;
    for color in &[Color::Red, Color::Green, Color::Blue, Color::Yellow, Color::Magenta, Color::Cyan, Color::White] {
        set_background_color(*color)?;
        print("Colored background ")?;
    }
    reset_color()?;
    println("")?;

    // Test attributes
    println("\nTesting attributes:")?;
    set_attribute(Attribute::Bright)?;
    println("Bright text")?;
    set_attribute(Attribute::Dim)?;
    println("Dim text")?;
    set_attribute(Attribute::Underscore)?;
    println("Underscored text")?;
    set_attribute(Attribute::Blink)?;
    println("Blinking text")?;
    set_attribute(Attribute::Reverse)?;
    println("Reversed text")?;
    reset_attributes()?;

    // Test cursor movement
    println("\nTesting cursor movement:")?;
    move_cursor_to(5, 15)?;
    println("This text starts at column 5, row 15")?;

    // Test clearing functions
    print("\nTesting clearing functions (3 second delay between each):")?;
    wait_for_seconds(3);
    clear_to_line_end()?;
    print("Cleared to line end")?;
    wait_for_seconds(3);
    clear_to_line_start()?;
    print("Cleared to line start")?;
    wait_for_seconds(3);
    clear_line()?;
    print("Cleared entire line")?;
    wait_for_seconds(3);
    clear_to_screen_end()?;
    println("Cleared to screen end")?;
    wait_for_seconds(3);
    clear_to_screen_start()?;
    println("Cleared to screen start")?;
    wait_for_seconds(3);

    // Test cursor save/restore
    save_cursor_location()?;
    move_cursor_to(10, 20)?;
    println("Cursor moved")?;
    wait_for_seconds(2);
    restore_cursor_location()?;
    println("Cursor restored")?;

    // Test input functions
    show_cursor()?;
    println("\nTesting input functions:")?;
    print("Press any key: ")?;
    let key = read_key()?;
    println(format!("\nYou pressed: {}", key).as_str())?;

    print("Enter a character: ")?;
    let ch = read_char()?;
    println(format!("\nYou entered: {}", ch).as_str())?;

    print("Enter a line of text: ")?;
    let line = read_line()?;
    println(format!("\nYou entered: {}", line).as_str())?;

    // Cleanup
    reset_color()?;
    reset_attributes()?;
    show_cursor()?;
    move_cursor_to(1, 25)?;
    println("Test completed. Press any key to exit.")?;
    let _ = read_key()?;

    Ok(())
}
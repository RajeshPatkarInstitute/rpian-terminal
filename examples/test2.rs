use rbox::{draw_box, hide_box, BoxStyle};
use rpian_terminal::*;
use std::io;

fn main() -> io::Result<()> {
    println("Welcome to the rpian-terminal feature test program!")?;
    println("Press Enter to start each test.")?;

    // Test viewport
    read_line()?;
    test_viewport()?;

    // Test colors
    read_line()?;
    test_colors()?;

    // Test attributes
    read_line()?;
    test_attributes()?;

    // Test cursor movement
    read_line()?;
    test_cursor_movement()?;

    // Test screen clearing
    read_line()?;
    test_screen_clearing()?;

    // Test input
    read_line()?;
    test_input()?;

    // Test waits
    read_line()?;
    test_waits()?;

    // Test box drawing
    read_line()?;
    test_box_drawing()?;

    // Test diagonal line
    read_line()?;
    test_diagonal_line()?;

    println("All tests completed. Press Enter to exit.")?;
    read_line()?;
    clear_screen()?;
    Ok(())
}

fn test_viewport() -> io::Result<()> {
    clear_screen()?;
    println("Testing viewport functions")?;
    set_viewport(80, 24);
    let (width, height) = get_viewport();
    println(&format!("Current viewport: {}x{}", width, height))?;
    Ok(())
}

fn test_colors() -> io::Result<()> {
    clear_screen()?;
    println("Testing color functions")?;
    for color in &[
        Color::Red,
        Color::Green,
        Color::Blue,
        Color::Yellow,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ] {
        set_foreground_color(*color)?;
        println(&format!("This text is in {:?}", color))?;
    }
    reset_color()?;
    println("Color test completed")?;
    Ok(())
}

fn test_attributes() -> io::Result<()> {
    clear_screen()?;
    println("Testing attribute functions")?;
    set_attribute(Attribute::Bright)?;
    println("This text is bright")?;
    set_attribute(Attribute::Dim)?;
    println("This text is dim")?;
    set_attribute(Attribute::Underscore)?;
    println("This text is underscored")?;
    set_attribute(Attribute::Blink)?;
    println("This text is blinking")?;
    set_attribute(Attribute::Reverse)?;
    println("This text is reversed")?;
    reset_attributes()?;
    println("Attribute test completed")?;
    Ok(())
}

fn test_cursor_movement() -> io::Result<()> {
    clear_screen()?;
    println("Testing cursor movement")?;
    move_cursor_to(10, 5)?;
    println("This text starts at column 10, row 5")?;
    save_cursor_location()?;
    move_cursor_to(1, 10)?;
    println("Now we're at the start of row 10")?;
    restore_cursor_location()?;
    println("And now we're back where we saved")?;
    Ok(())
}

fn test_screen_clearing() -> io::Result<()> {
    clear_screen()?;
    println("Testing screen clearing functions")?;
    println("This is some text")?;
    println("We'll clear to the end of the line after 'text': ")?;
    move_cursor_to(46, 2)?;
    clear_to_line_end()?;
    move_cursor_to(1, 4)?;
    println("Now we'll clear to the start of the line: ")?;
    move_cursor_to(1, 4)?;
    clear_to_line_start()?;
    move_cursor_to(1, 6)?;
    println("This entire line will be cleared")?;
    clear_line()?;
    move_cursor_to(1, 8)?;
    println("Everything above this line will be cleared")?;
    read_line()?;
    clear_to_screen_start()?;
    move_cursor_to(1, 10)?;
    println("Everything below this line will be cleared")?;
    read_line()?;
    clear_to_screen_end()?;
    Ok(())
}

fn test_input() -> io::Result<()> {
    clear_screen()?;
    println("Testing input functions")?;
    println("Press any key:")?;
    let key = read_key()?;
    println(&format!("You pressed: {}", key))?;
    println("Now enter a line of text:")?;
    let line = read_line()?;
    println(&format!("You entered: {}", line))?;
    Ok(())
}

fn test_waits() -> io::Result<()> {
    clear_screen()?;
    println("Testing wait functions")?;
    println("Waiting for 2 seconds...")?;
    wait_for_seconds(2);
    println("Waiting for 1000 milliseconds...")?;
    wait_for_millis(1000);
    println("Waiting for 1000000 microseconds...")?;
    wait_for_micros(1000000);
    println("Wait test completed")?;
    Ok(())
}

fn test_box_drawing() -> io::Result<()> {
    clear_screen()?;
    println("Testing box drawing functions")?;

    // Single line box without shading
    draw_box(5, 3, 20, 5, BoxStyle::Single)?;

    // Double line box with light shading
    draw_box(30, 3, 20, 5, BoxStyle::Double)?;

    // Single rounded box with medium shading
    draw_box(5, 10, 20, 5, BoxStyle::SingleRounded)?;

    // Double rounded box with dark shading
    set_foreground_color(Color::Red)?;
    draw_box(30, 10, 20, 5, BoxStyle::DoubleRounded)?;

    wait_for_seconds(4);

    // Hide the last box
    hide_box(30, 10, 20, 5)?;

    wait_for_seconds(2);

    // Redraw the last box with solid shading
    move_cursor_to(1, 17)?;
    draw_box(30, 10, 20, 5, BoxStyle::DoubleRounded)?;

    reset_color()?;
    println("Box drawing test completed")?;
    Ok(())
}

fn test_diagonal_line() -> io::Result<()> {
    clear_screen()?;
    println("Testing diagonal line function")?;
    diagonal_line('*', 1, 1, 20, 20)?;
    diagonal_line('#', 1, 20, 20, 1)?;
    move_cursor_to(1, 22)?;
    println("Diagonal line test completed")?;
    Ok(())
}

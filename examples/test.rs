use rpian_terminal::*;
use std::io;

fn prompt_continue() -> io::Result<()> {
    print("Press Enter to continue...")?;
    let _ = read_line()?;
    Ok(())
}

fn main() -> io::Result<()> {
    // Set up the viewport
    set_viewport(80, 24);
    clear_screen()?;

    // Display title
    set_foreground_color(Color::Green)?;
    set_attribute(Attribute::Bright)?;
    move_cursor_to(25, 1)?;
    println("rpian-terminal Interactive Demonstration")?;
    reset_color()?;
    reset_attributes()?;
    prompt_continue()?;

    // Demonstrate box drawing
    clear_screen()?;
    println("Box Drawing Demonstration:")?;
    draw_box(5, 3, 70, 18, BoxStyle::Double)?;
    draw_box(10, 5, 60, 14, BoxStyle::Single)?;
    move_cursor_to(1, 22)?;
    prompt_continue()?;

    // Demonstrate colors
    clear_screen()?;
    println("Color Demonstration:")?;
    for (i, color) in [Color::Red, Color::Green, Color::Blue, Color::Yellow, Color::Magenta, Color::Cyan, Color::White].iter().enumerate() {
        set_foreground_color(*color)?;
        move_cursor_to(5, 3 + i as u16)?;
        println(&format!("This is {} text", format!("{:?}", color)))?;
    }
    reset_color()?;
    move_cursor_to(1, 22)?;
    prompt_continue()?;

    // Demonstrate background colors
    clear_screen()?;
    println("Background Color Demonstration:")?;
    for (i, color) in [Color::Red, Color::Green, Color::Blue, Color::Yellow, Color::Magenta, Color::Cyan, Color::White].iter().enumerate() {
        set_background_color(*color)?;
        move_cursor_to(5, 3 + i as u16)?;
        println(&format!("This has a {:?} background", color))?;
    }
    reset_color()?;
    move_cursor_to(1, 22)?;
    prompt_continue()?;

    // Demonstrate text attributes
    clear_screen()?;
    println("Text Attributes Demonstration:")?;
    move_cursor_to(5, 3)?;
    set_attribute(Attribute::Bright)?;
    println("This text is bright")?;
    move_cursor_to(5, 4)?;
    set_attribute(Attribute::Dim)?;
    println("This text is dim")?;
    move_cursor_to(5, 5)?;
    set_attribute(Attribute::Underscore)?;
    println("This text is underscored")?;
    move_cursor_to(5, 6)?;
    set_attribute(Attribute::Blink)?;
    println("This text is blinking (if supported by your terminal)")?;
    move_cursor_to(5, 7)?;
    set_attribute(Attribute::Reverse)?;
    println("This text is reversed")?;
    reset_attributes()?;
    move_cursor_to(1, 22)?;
    prompt_continue()?;

    // Demonstrate line drawing
    clear_screen()?;
    println("Line Drawing Demonstration:")?;
    horizontal_line(BoxChar::Horizontal(BoxStyle::Single), 5, 3, 20)?;
    vertical_line(BoxChar::Vertical(BoxStyle::Single), 5, 4, 5)?;
    diagonal_line('*', 5, 10, 25, 15)?;
    move_cursor_to(1, 22)?;
    prompt_continue()?;

    // Demonstrate cursor movement and input
    clear_screen()?;
    println("Cursor Movement and Input Demonstration:")?;
    move_cursor_to(5, 3)?;
    print("Enter your name: ")?;
    let name = read_line()?;
    move_cursor_to(5, 5)?;
    set_foreground_color(Color::Cyan)?;
    println(&format!("Hello, {}!", name))?;
    reset_color()?;
    move_cursor_to(1, 22)?;
    prompt_continue()?;

    // Demonstrate waiting and clearing
    clear_screen()?;
    println("Waiting and Clearing Demonstration:")?;
    move_cursor_to(5, 3)?;
    println("This line will be cleared after 3 seconds.")?;
    wait_for_seconds(3);
    move_cursor_to(5, 3)?;
    clear_line()?;
    move_cursor_to(5, 3)?;
    println("The line has been cleared!")?;
    move_cursor_to(1, 22)?;
    prompt_continue()?;

    // Clean up and exit
    clear_screen()?;
    move_cursor_to(1, 1)?;
    println("Thank you for exploring rpian-terminal!")?;
    println("Press any key to exit...")?;
    let _ = read_key()?;

    Ok(())
}
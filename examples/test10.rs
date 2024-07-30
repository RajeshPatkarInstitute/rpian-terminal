use rpian_terminal::*;
use rbox::{draw_box, BoxStyle};

fn main() -> std::io::Result<()> {
    set_viewport(80, 24);
    clear_screen()?;
    
    set_foreground_color(Color::Green)?;
    set_attribute(Attribute::Bright)?;
    println("Welcome to rpian-terminal!")?;
    reset_color()?;
    reset_attributes()?;

    draw_box(5, 3, 70, 18, BoxStyle::Double)?;
    draw_box(10, 5, 60, 14, BoxStyle::SingleRounded)?;

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
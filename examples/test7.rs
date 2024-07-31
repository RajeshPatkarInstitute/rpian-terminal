use rbox::{draw_box, draw_shaded_rectangle, BoxStyle, ShadeStyle};
use rpian_terminal::*;

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

    draw_shaded_rectangle(15, 8, 50, 8, ShadeStyle::Medium);

    move_cursor_to(20, 10);
    print("Enter your name: ");
    let name = read_line();

    move_cursor_to(20, 12);
    set_foreground_color(Color::Blue);
    set_attribute(Attribute::Underscore);
    println(&format!("Hello, {}!", name));

    wait_for_seconds(2);
    clear_screen();
}
use rpian_terminal::*;
use rbox::{draw_box, BoxStyle};
use line::{Line, Direction,Shape};

fn main() {
    set_viewport(80, 24);
    clear_screen();
    hide_cursor();
    set_foreground_color(Color::Green);
    set_attribute(Attribute::Bright);
    println("Welcome to rpian-terminal!");
    reset_color();
    reset_attributes();

    draw_box(5, 3, 70, 18, BoxStyle::Double);
    draw_box(10, 5, 60, 14, BoxStyle::SingleRounded);

    let mut line = Line::new();
    line.x = 15;
    line.y = 10;
    line.size = 20;
    line.direction = Direction::East;
    line.show(Some(2));

    move_cursor_to(15, 12);
    print("Enter your name: ");
    let name = read_line();

    move_cursor_to(15, 14);
    set_foreground_color(Color::Blue);
    set_attribute(Attribute::Underscore);
    println(&format!("Hello, {}!", name));

    wait_for_seconds(2);
    show_cursor();
    clear_screen();
}
use rpian_terminal::*;
use line::{diagonal_line,*};

fn main() {
    set_viewport(150, 80);
    clear_screen();

    println("Line Module Test");

    // Test horizontal lines
    horizontal_line(5, 3, 20, HorizontalLineStyle::Light);
    horizontal_line(5, 5, 20, HorizontalLineStyle::Heavy);
    horizontal_line(5, 7, 20, HorizontalLineStyle::Double);

    // Test vertical lines
    vertical_line(30, 3, 5, VerticalLineStyle::Light);
    vertical_line(32, 3, 5, VerticalLineStyle::Heavy);
    vertical_line(34, 3, 5, VerticalLineStyle::Double);

    // Test diagonal lines
    diagonal_line(80, 23, 5, &DiagonalLineStyle::ForwardDiagonal);
    diagonal_line(90, 23, 5, &DiagonalLineStyle::BackwardDiagonal);
    diagonal_line(100,23, 5, &DiagonalLineStyle::ForwardSlash);
    diagonal_line(110, 23, 5, &DiagonalLineStyle::BackwardSlash);

    // Display legend
    move_cursor_to(5, 15);
    println("Horizontal Lines: Light, Heavy, Double");
    move_cursor_to(5, 16);
    println("Vertical Lines: Light, Heavy, Double");
    move_cursor_to(5, 27);
    println("Diagonal Lines: Forward, Backward, Forward Slash, Backward Slash");

    move_cursor_to(0, 40);
    println("Press any key to exit...");
    read_key();
    clear_screen();
}

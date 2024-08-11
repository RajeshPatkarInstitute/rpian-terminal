use rpian_terminal::{set_viewport,line::*};

fn main(){
    set_viewport(80,60);
    let mut line = Line::new();

    line.show(Some(2));  // Draws a light horizontal line

    line.direction = Direction::North;
    line.size = 15;
    line.show(Some(2));  // Draws a heavy horizontal line, now 15 units long

    line.direction = Direction::NorthEast;
    line.show(Some(2));  // Draws a diagonal line going northeast

    line.direction = Direction::SouthEast;
    line.show(Some(2));  // Draws a diagonal line going northeast
}
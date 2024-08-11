use rpian_terminal::*;
use rpian_terminal::line::{self,*};

fn main(){
    set_viewport(150,150);
    let speed = 1;
    clear_screen();
    hide_cursor();
    let mut l1 = line::Line::new();
    l1.style.enabless= true;
    l1.style.enabless= false;
    l1.x = 20;
    l1.y = 20;
    l1.show(Some(speed));
    l1.direction = Direction::West;
    l1.show(Some(speed));
    l1.direction = Direction::North;
    l1.show(None);
    wait_for_millis(speed as u64);
    l1.hide();
    l1.direction = Direction::South;
    l1.show(Some(speed));
    l1.direction = Direction::NorthEast;
    l1.show(Some(speed));
    l1.direction = Direction::SouthEast;
    l1.show(None);
    wait_for_millis(speed as u64);
    l1.hide();
    l1.direction = Direction::NorthWest;
    l1.show(None);
    wait_for_millis(speed as u64);
    l1.hide();
    l1.direction = Direction::SouthWest;
    l1.show(Some(speed));
    for v in 1..15{
        l1.show(Some(speed));
        l1.move_to(l1.x + v, l1.y);
    }
    show_cursor(); 
}
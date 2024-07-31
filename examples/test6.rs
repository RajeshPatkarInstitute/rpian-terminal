use rpian_terminal::*;
use std::char;

#[derive(Clone, Copy)]
pub struct BraillePattern {
    dots: [[bool; 2]; 4],
}

impl BraillePattern {
    pub fn new() -> Self {
        BraillePattern {
            dots: [[false; 2]; 4],
        }
    }

    pub fn set_dot(&mut self, row: usize, col: usize, value: bool) {
        if row < 4 && col < 2 {
            self.dots[row][col] = value;
        }
    }

    pub fn to_char(&self) -> char {
        let mut value = 0x2800;
        let weights = [0x01, 0x08, 0x02, 0x10, 0x04, 0x20, 0x40, 0x80];
        for row in 0..4 {
            for col in 0..2 {
                if self.dots[row][col] {
                    value |= weights[row * 2 + col];
                }
            }
        }
        char::from_u32(value).unwrap_or(' ')
    }
}

fn char_to_braille(c: char) -> BraillePattern {
    let mut pattern = BraillePattern::new();
    match c.to_lowercase().next().unwrap() {
        'a' => pattern.set_dot(0, 0, true),
        'b' => { pattern.set_dot(0, 0, true); pattern.set_dot(1, 0, true); },
        'c' => { pattern.set_dot(0, 0, true); pattern.set_dot(0, 1, true); },
        'd' => { pattern.set_dot(0, 0, true); pattern.set_dot(0, 1, true); pattern.set_dot(1, 1, true); },
        'e' => { pattern.set_dot(0, 0, true); pattern.set_dot(1, 1, true); },
        'f' => { pattern.set_dot(0, 0, true); pattern.set_dot(0, 1, true); pattern.set_dot(1, 0, true); },
        'g' => { pattern.set_dot(0, 0, true); pattern.set_dot(0, 1, true); pattern.set_dot(1, 0, true); pattern.set_dot(1, 1, true); },
        'h' => { pattern.set_dot(0, 0, true); pattern.set_dot(1, 0, true); pattern.set_dot(1, 1, true); },
        'i' => { pattern.set_dot(0, 1, true); pattern.set_dot(1, 0, true); },
        'j' => { pattern.set_dot(0, 1, true); pattern.set_dot(1, 0, true); pattern.set_dot(1, 1, true); },
        'k' => { pattern.set_dot(0, 0, true); pattern.set_dot(2, 0, true); },
        'l' => { pattern.set_dot(0, 0, true); pattern.set_dot(1, 0, true); pattern.set_dot(2, 0, true); },
        'm' => { pattern.set_dot(0, 0, true); pattern.set_dot(0, 1, true); pattern.set_dot(2, 0, true); },
        'n' => { pattern.set_dot(0, 0, true); pattern.set_dot(0, 1, true); pattern.set_dot(1, 1, true); pattern.set_dot(2, 0, true); },
        'o' => { pattern.set_dot(0, 0, true); pattern.set_dot(1, 1, true); pattern.set_dot(2, 0, true); },
        'p' => { pattern.set_dot(0, 0, true); pattern.set_dot(0, 1, true); pattern.set_dot(1, 0, true); pattern.set_dot(2, 0, true); },
        'q' => { pattern.set_dot(0, 0, true); pattern.set_dot(0, 1, true); pattern.set_dot(1, 0, true); pattern.set_dot(1, 1, true); pattern.set_dot(2, 0, true); },
        'r' => { pattern.set_dot(0, 0, true); pattern.set_dot(1, 0, true); pattern.set_dot(1, 1, true); pattern.set_dot(2, 0, true); },
        's' => { pattern.set_dot(0, 1, true); pattern.set_dot(1, 0, true); pattern.set_dot(2, 0, true); },
        't' => { pattern.set_dot(0, 1, true); pattern.set_dot(1, 0, true); pattern.set_dot(1, 1, true); pattern.set_dot(2, 0, true); },
        'u' => { pattern.set_dot(0, 0, true); pattern.set_dot(2, 0, true); pattern.set_dot(2, 1, true); },
        'v' => { pattern.set_dot(0, 0, true); pattern.set_dot(1, 0, true); pattern.set_dot(2, 0, true); pattern.set_dot(2, 1, true); },
        'w' => { pattern.set_dot(0, 1, true); pattern.set_dot(1, 0, true); pattern.set_dot(1, 1, true); pattern.set_dot(2, 1, true); },
        'x' => { pattern.set_dot(0, 0, true); pattern.set_dot(0, 1, true); pattern.set_dot(2, 0, true); pattern.set_dot(2, 1, true); },
        'y' => { pattern.set_dot(0, 0, true); pattern.set_dot(0, 1, true); pattern.set_dot(1, 1, true); pattern.set_dot(2, 0, true); pattern.set_dot(2, 1, true); },
        'z' => { pattern.set_dot(0, 0, true); pattern.set_dot(1, 1, true); pattern.set_dot(2, 0, true); pattern.set_dot(2, 1, true); },
        ' ' => {},
        _ => {},
    }
    pattern
}

fn print_braille(text: &str) {
    for c in text.chars() {
        let braille = char_to_braille(c);
        print(&braille.to_char().to_string());
    }
}

fn main() {
    clear_screen();
    move_cursor_to(10, 10);
    println("Hello World");
    move_cursor_to(10, 11);
    print_braille("Hello World");
    move_cursor_to(0, 22); // Move cursor to bottom for clean exit
}
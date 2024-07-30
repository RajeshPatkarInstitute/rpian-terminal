use rpian_terminal::*;
use std::char;
use std::io::{self, Write};

/// Represents a 2x4 grid of dots for a Braille character
#[derive(Clone, Copy)]
pub struct BraillePattern {
    dots: [[bool; 2]; 4],
}

impl BraillePattern {
    /// Creates a new empty Braille pattern
    pub fn new() -> Self {
        BraillePattern {
            dots: [[false; 2]; 4],
        }
    }

    /// Sets a dot in the Braille pattern
    pub fn set_dot(&mut self, row: usize, col: usize, value: bool) {
        if row < 4 && col < 2 {
            self.dots[row][col] = value;
        }
    }

    /// Converts the Braille pattern to a Unicode character
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

/// Draws a smooth line using Braille characters
pub fn draw_braille_line(x1: f32, y1: f32, x2: f32, y2: f32) -> io::Result<()> {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let steps = dx.abs().max(dy.abs()) * 2.0;

    for i in 0..=steps as usize {
        let t = i as f32 / steps;
        let x = x1 + dx * t;
        let y = y1 + dy * t;

        let char_x = (x / 2.0).floor() as u16;
        let char_y = (y / 4.0).floor() as u16;
        let dot_x = (x % 2.0).round() as usize;
        let dot_y = (y % 4.0).round() as usize;

        move_cursor_to(char_x, char_y)?;

        let mut pattern = BraillePattern::new();
        pattern.set_dot(dot_y, dot_x, true);

        write!(io::stdout(), "{}", pattern.to_char())?;
    }

    io::stdout().flush()
}

/// Draws a filled circle using Braille characters
pub fn draw_braille_circle(
    center_x: f32,
    center_y: f32,
    radius: f32,
    filled: bool,
) -> io::Result<()> {
    let mut patterns = std::collections::HashMap::new();

    for angle in 0..360 {
        let radian = angle as f32 * std::f32::consts::PI / 180.0;
        let x = center_x + radius * radian.cos();
        let y = center_y + radius * radian.sin();

        let char_x = (x / 2.0).floor() as u16;
        let char_y = (y / 4.0).floor() as u16;
        let dot_x = (x % 2.0).round() as usize;
        let dot_y = (y % 4.0).round() as usize;

        patterns
            .entry((char_x, char_y))
            .or_insert_with(BraillePattern::new)
            .set_dot(dot_y, dot_x, true);

        if filled {
            for r in 1..=(radius as usize) {
                let fill_x = center_x + (r as f32) * radian.cos();
                let fill_y = center_y + (r as f32) * radian.sin();
                let fill_char_x = (fill_x / 2.0).floor() as u16;
                let fill_char_y = (fill_y / 4.0).floor() as u16;
                let fill_dot_x = (fill_x % 2.0).round() as usize;
                let fill_dot_y = (fill_y % 4.0).round() as usize;

                patterns
                    .entry((fill_char_x, fill_char_y))
                    .or_insert_with(BraillePattern::new)
                    .set_dot(fill_dot_y, fill_dot_x, true);
            }
        }
    }

    for ((x, y), pattern) in patterns {
        move_cursor_to(x, y)?;
        write!(io::stdout(), "{}", pattern.to_char())?;
    }

    io::stdout().flush()
}

fn main() -> io::Result<()> {
    clear_screen()?;

    // Draw a smooth diagonal line
    draw_braille_line(0.0, 0.0, 40.0, 20.0)?;

    // Draw an outline circle
    draw_braille_circle(30.0, 10.0, 8.0, false)?;

    // Draw a filled circle
    draw_braille_circle(50.0, 15.0, 6.0, true)?;

    move_cursor_to(0, 22)?; // Move cursor to bottom for clean exit
    Ok(())
}

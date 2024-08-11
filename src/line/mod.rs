use circle::circle_symbol_to_char;
use star::star_symbol_to_char;

use crate::*;
use crate::move_cursor_to;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum HorizontalLineStyle {
    Light,
    Heavy,
    Double,
    LightTripleDash,
    HeavyTripleDash,
    LightQuadrupleDash,
    HeavyQuadrupleDash,
    Dotted,
    LightDoubleDash,
    HeavyDoubleDash,
    Wavy,
    Space,
}

pub fn get_horizontal_line_char(style: &HorizontalLineStyle) -> char {
    match style {
        HorizontalLineStyle::Light => '─',
        HorizontalLineStyle::Heavy => '━',
        HorizontalLineStyle::Double => '═',
        HorizontalLineStyle::LightTripleDash => '┄',
        HorizontalLineStyle::HeavyTripleDash => '┅',
        HorizontalLineStyle::LightQuadrupleDash => '┈',
        HorizontalLineStyle::HeavyQuadrupleDash => '┉',
        HorizontalLineStyle::Dotted => '·',
        HorizontalLineStyle::LightDoubleDash => '╌',
        HorizontalLineStyle::HeavyDoubleDash => '╍',
        HorizontalLineStyle::Wavy => '﹉',
        HorizontalLineStyle::Space => ' ',
    }
}

pub fn horizontal_line(x: u16, y: u16, size: usize, style: HorizontalLineStyle) {
    let (viewport_width, viewport_height) = get_viewport();
    if x >= viewport_width || y >= viewport_height {
        handle_boundary_error("Line start position is outside viewport");
        return;
    }
    if x + size as u16 > viewport_width {
        handle_boundary_error("Line extends beyond viewport width");
        return;
    }

    let line_char = get_horizontal_line_char(&style);
    move_cursor_to(x, y);

    for _ in 0..size {
        put_char(line_char);
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum VerticalLineStyle {
    Light,
    Heavy,
    Double,
    LightTripleDash,
    HeavyTripleDash,
    LightQuadrupleDash,
    HeavyQuadrupleDash,
    Dotted,
    LightDoubleDash,
    HeavyDoubleDash,
    Space,
}

pub fn get_vertical_line_char(style: &VerticalLineStyle) -> char {
    match style {
        VerticalLineStyle::Light => '│',
        VerticalLineStyle::Heavy => '┃',
        VerticalLineStyle::Double => '║',
        VerticalLineStyle::LightTripleDash => '┆',
        VerticalLineStyle::HeavyTripleDash => '┇',
        VerticalLineStyle::LightQuadrupleDash => '┊',
        VerticalLineStyle::HeavyQuadrupleDash => '┋',
        VerticalLineStyle::Dotted => '·',
        VerticalLineStyle::LightDoubleDash => '╎',
        VerticalLineStyle::HeavyDoubleDash => '╏',
        VerticalLineStyle::Space => ' ',
    }
}

pub fn vertical_line(x: u16, y: u16, size: usize, style: VerticalLineStyle) {
    let (viewport_width, viewport_height) = get_viewport();
    if x >= viewport_width || y >= viewport_height {
        handle_boundary_error("Line start position is outside viewport");
        return;
    }
    if y + size as u16 > viewport_height {
        handle_boundary_error("Line extends beyond viewport height");
        return;
    }

    let line_char = get_vertical_line_char(&style);
    for i in 0..size {
        move_cursor_to(x, y + i as u16);
        put_char(line_char);
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DiagonalLineStyle {
    ForwardDiagonal,  // U+2571: ╱
    BackwardDiagonal, // U+2572: ╲
    ForwardSlash,     // /
    BackwardSlash,    // \
    Space,
}

pub fn get_diagonal_line_char(style: &DiagonalLineStyle) -> char {
    match style {
        DiagonalLineStyle::ForwardDiagonal => '╱',
        DiagonalLineStyle::BackwardDiagonal => '╲',
        DiagonalLineStyle::ForwardSlash => '/',
        DiagonalLineStyle::BackwardSlash => '\\',
        DiagonalLineStyle::Space => ' ',
    }
}

pub fn diagonal_line(x: u16, y: u16, size: usize, direction: Direction) {
    let (viewport_width, viewport_height) = get_viewport();
    if x >= viewport_width || y >= viewport_height {
        handle_boundary_error("Line start position is outside viewport");
        return;
    }

    let mut line_char;
    for i in 0..size {
        let (new_x, new_y) = match direction {
            Direction::NorthEast => {
                line_char = get_diagonal_line_char(&DiagonalLineStyle::BackwardDiagonal);
                (x - i as u16, y - i as u16)
            },
            Direction::NorthWest => {
                line_char = get_diagonal_line_char(&DiagonalLineStyle::ForwardDiagonal);
                (x + i as u16, y - i as u16)
            },
            Direction::SouthWest => {
                line_char = get_diagonal_line_char(&DiagonalLineStyle::BackwardDiagonal);
                (x + i as u16, y + i as u16)
            },
            Direction::SouthEast => {
                line_char = get_diagonal_line_char(&DiagonalLineStyle::BackwardDiagonal);
                (x + i as u16, y + i as u16)
            },
            _ => break,
        };

        if new_x >= viewport_width || new_y >= viewport_height || new_x < 1 || new_y < 1 {
            handle_boundary_error("Line extends beyond viewport");
            return;
        }

        move_cursor_to(new_x, new_y);
        put_char(line_char);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VerticeStyle {
    Star(star::StarSymbol),
    Circle(circle::CircleSymbol),
    Space,
}

pub fn get_vertice(v: &VerticeStyle) -> char {
    match v {
        VerticeStyle::Star(x) => star_symbol_to_char(*x),
        VerticeStyle::Circle(y) => circle_symbol_to_char(*y),
        VerticeStyle::Space => ' ',
    }
}

fn decrement(v: &mut u8) -> &mut u8 {
    *v -= 1;
    v
}

fn increment(v: &mut u8) -> &mut u8 {
    *v += 1;
    v
}

pub struct Line {
    pub x: u8,
    pub y: u8,
    pub size: usize,
    pub style: LineStyle,
    pub direction: Direction,
}

pub struct LineStyle {
    pub hs: HorizontalLineStyle,
    pub vs: VerticalLineStyle,
    pub ds: bool,
    pub ss: VerticeStyle,
    pub es: VerticeStyle,
    pub enabless: bool,
    pub enablees: bool,
}

impl Line {
    pub fn new() -> Self {
        Self {
            x: 1,
            y: 1,
            size: 10,
            style: LineStyle {
                hs: HorizontalLineStyle::Light,
                vs: VerticalLineStyle::Light,
                ds: true,
                ss: VerticeStyle::Circle(circle::CircleSymbol::FilledCircle),
                es: VerticeStyle::Circle(circle::CircleSymbol::FilledCircle),
                enabless: true,
                enablees: true,
            },
            direction: Direction::East,
        }
    }

    pub fn draw(&self, show: bool) {
        match self.direction {
            Direction::East => {
                draw(
                    self,
                    if show { get_horizontal_line_char(&self.style.hs) } else { ' ' },
                    |a, _| { increment(a); },
                );
            }
            Direction::West => {
                draw(
                    self,
                    if show { get_horizontal_line_char(&self.style.hs) } else { ' ' },
                    |a, _| { decrement(a); },
                );
            }
            Direction::North => {
                draw(
                    self,
                    if show { get_vertical_line_char(&self.style.vs) } else { ' ' },
                    |_, b| { decrement(b); },
                );
            }
            Direction::South => {
                draw(
                    self,
                    if show { get_vertical_line_char(&self.style.vs) } else { ' ' },
                    |_, b| { increment(b); },
                );
            }
            Direction::NorthWest => {
                draw(
                    self,
                    if show { get_diagonal_line_char(&DiagonalLineStyle::BackwardDiagonal) } else { ' ' },
                    |a, b| { decrement(a); decrement(b); },
                );
            }
            Direction::NorthEast => {
                draw(
                    self,
                    if show { get_diagonal_line_char(&DiagonalLineStyle::ForwardDiagonal) } else { ' ' },
                    |a, b| { increment(a); decrement(b); },
                );
            }
            Direction::SouthWest => {
                draw(
                    self,
                    if show { get_diagonal_line_char(&DiagonalLineStyle::ForwardDiagonal) } else { ' ' },
                    |a, b| { decrement(a); increment(b); },
                );
            }
            Direction::SouthEast => {
                draw(
                    self,
                    if show { get_diagonal_line_char(&DiagonalLineStyle::BackwardDiagonal) } else { ' ' },
                    |a, b| { increment(a); increment(b); },
                );
            }
        }

        fn draw(l: &Line, ch: char, op: fn(&mut u8, &mut u8)) {
            let mut current_x = l.x;
            let mut current_y = l.y;
            let mut i = 0;
            while i < l.size {
                move_cursor_to(current_x as u16, current_y as u16);
                if i == 0 && l.style.enabless {
                    put_char(if ch != ' ' { get_vertice(&l.style.ss) } else { ' ' })
                } else if i == l.size - 1 && l.style.enablees {
                    put_char(if ch != ' ' { get_vertice(&l.style.es) } else { ' ' });
                } else {
                    put_char(ch);
                }
                op(&mut current_x, &mut current_y);
                i += 1;
            }
        }
    }
}

pub trait Shape {
    fn show(&self, time: Option<u8>);
    fn hide(&self);
    fn move_to(&mut self, x: u8, y: u8);
}

impl Shape for Line {
    fn move_to(&mut self, x: u8, y: u8) {
        self.x = x;
        self.y = y;
    }

    fn show(&self, time: Option<u8>) {
        self.draw(true);
        match time {
            Some(x) => {
                wait_for_seconds(x as u64);
                self.hide();
            },
            None => {}
        }
    }

    fn hide(&self) {
        self.draw(false);
    }
}
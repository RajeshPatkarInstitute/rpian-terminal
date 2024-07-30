use std::io::{self, Write};

#[derive(Debug, PartialEq, Eq)]
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
    }
}

pub fn horizontal_line(x: u32, y: u32, size: usize, style: HorizontalLineStyle) {
    let line_char = get_horizontal_line_char(&style);
    move_cursor_to(x, y)?;

    for _ in 0..size {
        print!("{}", line_char);
    }
    io::stdout().flush().unwrap();
}

#[derive(Debug, PartialEq, Eq)]
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
    }
}

pub fn vertical_line(x: u32, y: u32, size: usize, style: VerticalLineStyle) {
    let line_char = get_vertical_line_char(&style);
    for i in 0..size {
        move_cursor_to(x, y + i as u32)?;
        print!("{}", line_char);
    }
    io::stdout().flush().unwrap();
}

#[derive(Debug, PartialEq, Eq)]
pub enum DiagonalLineStyle {
    ForwardDiagonal,  // U+2571: ╱
    BackwardDiagonal, // U+2572: ╲
    ForwardSlash,     // /
    BackwardSlash,    // \
}

pub fn get_diagonal_line_char(style: &DiagonalLineStyle) -> char {
    match style {
        DiagonalLineStyle::ForwardDiagonal => '╱',  // U+2571
        DiagonalLineStyle::BackwardDiagonal => '╲', // U+2572
        DiagonalLineStyle::ForwardSlash => '/',     // /
        DiagonalLineStyle::BackwardSlash => '\\',   // \
    }
}

pub fn diagonal_line(x: u32, y: u32, size: usize, style: &DiagonalLineStyle) {
    let line_char = get_diagonal_line_char(&style);
    for i in 0..size {
        match style {
            DiagonalLineStyle::BackwardDiagonal | DiagonalLineStyle::BackwardSlash => {
                move_cursor_to(x + i as u32, y + i as u32)?; // Down-right diagonal
            }
            DiagonalLineStyle::ForwardDiagonal | DiagonalLineStyle::ForwardSlash => {
                move_cursor_to(x + i as u32, y - i as u32)?; // Up-right diagonal
            }
        }
        print!("{}", line_char);
    }
    io::stdout().flush().unwrap();
}
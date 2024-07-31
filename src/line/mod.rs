use crate::*;

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

pub fn diagonal_line(x: u16, y: u16, size: usize, style: &DiagonalLineStyle) {
    let (viewport_width, viewport_height) = get_viewport();
    if x >= viewport_width || y >= viewport_height {
        handle_boundary_error("Line start position is outside viewport");
        return;
    }

    let line_char = get_diagonal_line_char(style);
    for i in 0..size {
        let (new_x, new_y) = match style {
            DiagonalLineStyle::BackwardDiagonal | DiagonalLineStyle::BackwardSlash => {
                (x + i as u16, y + i as u16)
            }
            DiagonalLineStyle::ForwardDiagonal | DiagonalLineStyle::ForwardSlash => {
                if y < i as u16 {
                    handle_boundary_error("Line extends beyond top of viewport");
                    return;
                }
                (x + i as u16, y - i as u16)
            }
        };

        if new_x >= viewport_width || new_y >= viewport_height {
            handle_boundary_error("Line extends beyond viewport");
            return;
        }

        move_cursor_to(new_x, new_y);
        put_char(line_char);
    }
}